use chrono::{DateTime, Utc};
use crate::shared::error::DomainError;
use crate::shared::event::DomainEvent;
use crate::id;

use super::value_objects::{ConnectionConfig, DatabaseType, EncryptedPassword};

#[derive(Debug)]
pub struct Connection {
    id: String,
    name: String,
    database_type: DatabaseType,
    config: ConnectionConfig,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Connection {
    pub fn create(
        name: String,
        database_type: DatabaseType,
        config: ConnectionConfig,
    ) -> Result<(Self, Vec<DomainEvent>), DomainError> {
        if name.trim().is_empty() {
            return Err(DomainError::InvalidConnectionConfig(
                "Connection name cannot be empty".to_string(),
            ));
        }

        let now = Utc::now();
        let id = id::generate_id_string();

        let connection = Self {
            id: id.clone(),
            name,
            database_type: database_type.clone(),
            config,
            created_at: now,
            updated_at: now,
        };

        let events = vec![DomainEvent::ConnectionCreated {
            connection_id: id,
            database_type: database_type.to_string(),
            timestamp: now,
        }];

        Ok((connection, events))
    }

    pub fn reconstitute(
        id: String,
        name: String,
        database_type: DatabaseType,
        config: ConnectionConfig,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            name,
            database_type,
            config,
            created_at,
            updated_at,
        }
    }

    pub fn update(
        mut self,
        name: Option<String>,
        config: Option<ConnectionConfig>,
    ) -> Result<(Self, Vec<DomainEvent>), DomainError> {
        let mut changed_fields = Vec::new();

        if let Some(name) = name {
            if name.trim().is_empty() {
                return Err(DomainError::InvalidConnectionConfig(
                    "Connection name cannot be empty".to_string(),
                ));
            }
            if self.name != name {
                changed_fields.push("name".to_string());
                self.name = name;
            }
        }

        if let Some(config) = config
            && self.config != config
        {
            changed_fields.push("config".to_string());
            self.config = config;
        }

        let now = Utc::now();
        self.updated_at = now;

        let events = if changed_fields.is_empty() {
            Vec::new()
        } else {
            vec![DomainEvent::ConnectionUpdated {
                connection_id: self.id.clone(),
                changed_fields,
                timestamp: now,
            }]
        };

        Ok((self, events))
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn database_type(&self) -> &DatabaseType {
        &self.database_type
    }

    pub fn config(&self) -> &ConnectionConfig {
        &self.config
    }

    pub fn host(&self) -> &str {
        &self.config.host
    }

    pub fn port(&self) -> u16 {
        self.config.port
    }

    pub fn database(&self) -> &str {
        &self.config.database
    }

    pub fn username(&self) -> &str {
        &self.config.username
    }

    pub fn encrypted_password(&self) -> &EncryptedPassword {
        &self.config.encrypted_password
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::value_objects::{ConnectionConfig, EncryptedPassword};

    fn valid_config() -> ConnectionConfig {
        ConnectionConfig {
            host: "localhost".to_string(),
            port: 5432,
            database: "testdb".to_string(),
            username: "admin".to_string(),
            encrypted_password: EncryptedPassword::new("encrypted_test".to_string()),
        }
    }

    #[test]
    fn test_create_connection_success() {
        let result = Connection::create(
            "My Connection".to_string(),
            DatabaseType::Postgres,
            valid_config(),
        );
        assert!(result.is_ok());
        let (conn, events) = result.unwrap();
        assert_eq!(conn.name(), "My Connection");
        assert_eq!(conn.database_type(), &DatabaseType::Postgres);
        assert_eq!(conn.host(), "localhost");
        assert_eq!(conn.port(), 5432);
        assert!(!conn.id().is_empty());
        assert_eq!(events.len(), 1);
    }

    #[test]
    fn test_create_connection_empty_name() {
        let result = Connection::create(
            "  ".to_string(),
            DatabaseType::Postgres,
            valid_config(),
        );
        assert!(result.is_err());
        match result.unwrap_err() {
            DomainError::InvalidConnectionConfig(msg) => {
                assert!(msg.contains("empty"));
            }
            _ => panic!("Expected InvalidConnectionConfig"),
        }
    }

    #[test]
    fn test_update_connection_name() {
        let (conn, _) = Connection::create(
            "Old Name".to_string(),
            DatabaseType::Mysql,
            valid_config(),
        )
        .unwrap();

        let result = conn.update(Some("New Name".to_string()), None);
        assert!(result.is_ok());
        let (updated, events) = result.unwrap();
        assert_eq!(updated.name(), "New Name");
        assert_eq!(events.len(), 1);
    }

    #[test]
    fn test_update_connection_empty_name() {
        let (conn, _) = Connection::create(
            "Old Name".to_string(),
            DatabaseType::Mysql,
            valid_config(),
        )
        .unwrap();

        let result = conn.update(Some("  ".to_string()), None);
        assert!(result.is_err());
    }

    #[test]
    fn test_update_connection_no_changes() {
        let (conn, _) = Connection::create(
            "Same Name".to_string(),
            DatabaseType::Sqlite,
            valid_config(),
        )
        .unwrap();

        let result = conn.update(Some("Same Name".to_string()), None);
        assert!(result.is_ok());
        let (_, events) = result.unwrap();
        assert_eq!(events.len(), 0);
    }
}