use crate::shared::error::DomainError;
use crate::shared::crypto::EncryptionService;
use crate::shared::infrastructure_error::InfrastructureError;

pub use sql_admin_api_types::DatabaseType;

#[derive(Clone, Debug, PartialEq)]
pub struct EncryptedPassword(String);

impl EncryptedPassword {
    pub fn new(encrypted: String) -> Self {
        Self(encrypted)
    }

    pub fn from_raw(
        plain: &str,
        crypto: &dyn EncryptionService,
    ) -> Result<Self, DomainError> {
        let encrypted = crypto
            .encrypt(plain)
            .map_err(|e| DomainError::InvalidConnectionConfig(format!("Encryption failed: {}", e)))?;
        Ok(Self(encrypted))
    }

    pub fn decrypt(
        &self,
        crypto: &dyn EncryptionService,
    ) -> Result<String, InfrastructureError> {
        crypto.decrypt(&self.0)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ConnectionConfig {
    pub host: String,
    pub port: u16,
    pub database: String,
    pub username: String,
    pub encrypted_password: EncryptedPassword,
}

impl ConnectionConfig {
    pub fn new(
        host: String,
        port: u16,
        database: String,
        username: String,
        encrypted_password: EncryptedPassword,
        db_type: DatabaseType,
    ) -> Result<Self, DomainError> {
        let is_file_based = matches!(db_type, DatabaseType::Sqlite | DatabaseType::Redb);

        if host.trim().is_empty() {
            return Err(DomainError::InvalidConnectionConfig(
                "Host cannot be empty".to_string(),
            ));
        }
        if !is_file_based && port == 0 {
            return Err(DomainError::InvalidConnectionConfig(
                "Port cannot be zero".to_string(),
            ));
        }
        if database.trim().is_empty() {
            return Err(DomainError::InvalidConnectionConfig(
                "Database cannot be empty".to_string(),
            ));
        }
        if !is_file_based && username.trim().is_empty() {
            return Err(DomainError::InvalidConnectionConfig(
                "Username cannot be empty".to_string(),
            ));
        }
        Ok(Self {
            host,
            port,
            database,
            username,
            encrypted_password,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::crypto::EncryptionService;
    use crate::shared::infrastructure_error::InfrastructureError;

    struct MockCrypto;
    #[async_trait::async_trait]
    impl EncryptionService for MockCrypto {
        fn encrypt(&self, plaintext: &str) -> Result<String, InfrastructureError> {
            Ok(format!("encrypted:{}", plaintext))
        }
        fn decrypt(&self, ciphertext: &str) -> Result<String, InfrastructureError> {
            Ok(ciphertext.strip_prefix("encrypted:").unwrap_or(ciphertext).to_string())
        }
    }

    #[test]
    fn test_connection_config_valid() {
        let crypto = MockCrypto;
        let password = EncryptedPassword::from_raw("secret", &crypto).unwrap();
        let config = ConnectionConfig::new(
            "localhost".to_string(),
            5432,
            "mydb".to_string(),
            "admin".to_string(),
            password,
            DatabaseType::Postgres,
        );
        assert!(config.is_ok());
    }

    #[test]
    fn test_connection_config_empty_host() {
        let encrypted = EncryptedPassword::new("enc".to_string());
        let config = ConnectionConfig::new(
            "".to_string(),
            5432,
            "mydb".to_string(),
            "admin".to_string(),
            encrypted,
            DatabaseType::Postgres,
        );
        assert!(config.is_err());
    }

    #[test]
    fn test_connection_config_zero_port() {
        let encrypted = EncryptedPassword::new("enc".to_string());
        let config = ConnectionConfig::new(
            "localhost".to_string(),
            0,
            "mydb".to_string(),
            "admin".to_string(),
            encrypted,
            DatabaseType::Postgres,
        );
        assert!(config.is_err());
    }

    #[test]
    fn test_connection_config_sqlite_skips_port_username_validation() {
        let encrypted = EncryptedPassword::new("enc".to_string());
        let config = ConnectionConfig::new(
            "localhost".to_string(),
            0,
            "mydb.sqlite".to_string(),
            "".to_string(),
            encrypted,
            DatabaseType::Sqlite,
        );
        assert!(config.is_ok());
    }

    #[test]
    fn test_connection_config_redb_skips_port_username_validation() {
        let encrypted = EncryptedPassword::new("enc".to_string());
        let config = ConnectionConfig::new(
            "localhost".to_string(),
            0,
            "mydb.redb".to_string(),
            "".to_string(),
            encrypted,
            DatabaseType::Redb,
        );
        assert!(config.is_ok());
    }

    #[test]
    fn test_encrypted_password_roundtrip() {
        let crypto = MockCrypto;
        let password =
            EncryptedPassword::from_raw("my_secret", &crypto).unwrap();
        let decrypted = password.decrypt(&crypto).unwrap();
        assert_eq!(decrypted, "my_secret");
    }
}