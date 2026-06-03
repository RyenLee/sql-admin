use std::sync::Arc;
use sql_admin_domain::connection::aggregate::Connection as DomainConnection;
use sql_admin_domain::connection::repository::ConnectionRepository;
use sql_admin_domain::connection::value_objects::{ConnectionConfig, EncryptedPassword};
use sql_admin_domain::shared::crypto::EncryptionService;
use sql_admin_domain::shared::event::EventBus;
use sql_admin_domain::shared::pool::PoolFactory;
use sql_admin_domain::shared::application_error::ApplicationError;

use crate::connection_pool_service::ConnectionPoolService;
use crate::dto::{
    Connection, CreateConnectionRequest, DeleteConnectionRequest, UpdateConnectionRequest,
};

pub struct ConnectionHandler {
    repo: Arc<dyn ConnectionRepository>,
    crypto: Arc<dyn EncryptionService>,
    event_bus: Arc<dyn EventBus>,
    pool_factory: Arc<dyn PoolFactory>,
    pool_service: ConnectionPoolService,
}

impl ConnectionHandler {
    pub fn new(
        repo: Arc<dyn ConnectionRepository>,
        crypto: Arc<dyn EncryptionService>,
        event_bus: Arc<dyn EventBus>,
        pool_factory: Arc<dyn PoolFactory>,
        pool_service: ConnectionPoolService,
    ) -> Self {
        Self { repo, crypto, event_bus, pool_factory, pool_service }
    }

    pub async fn create(
        &self,
        cmd: CreateConnectionRequest,
    ) -> Result<Connection, ApplicationError> {
        let encrypted_password = match cmd.password {
            Some(ref p) if !p.is_empty() => EncryptedPassword::from_raw(p, self.crypto.as_ref())?,
            _ => EncryptedPassword::new(String::new()),
        };

        let db_type = cmd.database_type;

        let config = ConnectionConfig::new(
            cmd.host,
            cmd.port,
            cmd.database,
            cmd.username,
            encrypted_password,
            db_type.clone(),
        )?;

        let (connection, events) = DomainConnection::create(cmd.name, db_type, config)?;

        self.repo.save(&connection).await?;

        for event in events {
            if let Err(e) = self.event_bus.publish(event).await {
                tracing::warn!(
                    module = "connection_handler",
                    event = "event_publish_failed",
                    error = %e,
                    "Failed to publish domain event"
                );
            }
        }

        Ok(domain_conn_to_dto(&connection))
    }

    pub async fn list(&self) -> Result<Vec<Connection>, ApplicationError> {
        let connections = self.repo.find_all().await?;
        Ok(connections.iter().map(domain_conn_to_dto).collect())
    }

    pub async fn get_by_id(&self, id: &str) -> Result<Option<Connection>, ApplicationError> {
        let conn = self.repo.find_by_id(id).await?;
        Ok(conn.as_ref().map(domain_conn_to_dto))
    }

    pub async fn update(
        &self,
        id: String,
        cmd: UpdateConnectionRequest,
    ) -> Result<Connection, ApplicationError> {
        let conn = self
            .repo
            .find_by_id(&id)
            .await?
            .ok_or_else(|| ApplicationError::NotFound(format!("Connection not found: {}", id)))?;

        let db_type = conn.database_type().clone();
        let config = if cmd.host.is_some() || cmd.port.is_some() || cmd.database.is_some() || cmd.username.is_some() || cmd.password.is_some() {
            let base_config = conn.config().clone();
            let host = cmd.host.unwrap_or_else(|| base_config.host.clone());
            let port = cmd.port.unwrap_or(base_config.port);
            let database = cmd.database.unwrap_or_else(|| base_config.database.clone());
            let username = cmd.username.unwrap_or_else(|| base_config.username.clone());
            let encrypted_password = match cmd.password {
                Some(ref p) if !p.is_empty() => EncryptedPassword::from_raw(p, self.crypto.as_ref())?,
                Some(_) => base_config.encrypted_password.clone(),
                None => base_config.encrypted_password.clone(),
            };
            Some(ConnectionConfig::new(host, port, database, username, encrypted_password, db_type)?)
        } else {
            None
        };

        let (updated, events) = conn.update(cmd.name, config)?;

        self.repo.save(&updated).await?;

        for event in events {
            if let Err(e) = self.event_bus.publish(event).await {
                tracing::warn!(
                    module = "connection_handler",
                    event = "event_publish_failed",
                    error = %e,
                    "Failed to publish connection updated event"
                );
            }
        }

        Ok(domain_conn_to_dto(&updated))
    }

    pub async fn delete(&self, cmd: DeleteConnectionRequest) -> Result<bool, ApplicationError> {
        let conn = self
            .repo
            .find_by_id(&cmd.id)
            .await?
            .ok_or_else(|| ApplicationError::NotFound(format!("Connection not found: {}", cmd.id)))?;

        let deleted = self.repo.delete(&cmd.id).await?;
        if !deleted {
            return Err(ApplicationError::NotFound(format!(
                "Connection not found: {}",
                cmd.id
            )));
        }

        let event = sql_admin_domain::shared::event::DomainEvent::ConnectionDeleted {
            connection_id: conn.id().to_string(),
            timestamp: chrono::Utc::now(),
        };
        if let Err(e) = self.event_bus.publish(event).await {
            tracing::warn!(
                module = "connection_handler",
                event = "event_publish_failed",
                error = %e,
                "Failed to publish connection deleted event"
            );
        }

        Ok(true)
    }

    pub async fn test_connection(&self, id: &str) -> Result<String, ApplicationError> {
        let _ = self
            .pool_service
            .get_executor(id)
            .await
            .map_err(|e| ApplicationError::Validation(format!("Connection test failed: {}", e)))?;

        Ok("Connection successful".to_string())
    }

    pub async fn test_connection_request(
        &self,
        req: CreateConnectionRequest,
    ) -> Result<String, ApplicationError> {
        let db_type = req.database_type;
        let password = req.password.unwrap_or_default();
        
        let result = self
            .pool_factory
            .create_pool(
                "test",
                &db_type,
                &req.host,
                req.port,
                &req.database,
                &req.username,
                &password,
            )
            .await
            .map_err(|e| ApplicationError::Validation(format!("Connection test failed: {}", e)))?;

        // 测试完成后清理临时连接池，释放 Redb 文件锁等资源
        drop(result);
        self.pool_factory.invalidate_pool("test").await;

        Ok("Connection successful".to_string())
    }
}

fn domain_conn_to_dto(c: &DomainConnection) -> Connection {
    Connection {
        id: c.id().to_string(),
        name: c.name().to_string(),
        database_type: c.database_type().clone(),
        host: c.host().to_string(),
        port: c.port(),
        database: c.database().to_string(),
        username: c.username().to_string(),
        password: None,
        created_at: c.created_at(),
        updated_at: c.updated_at(),
    }
}
