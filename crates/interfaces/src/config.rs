use anyhow::Result;
use std::env;

#[derive(Clone, Debug)]
pub enum Environment {
    Development,
    Production,
}

impl Environment {
    pub fn is_production(&self) -> bool {
        matches!(self, Environment::Production)
    }
}

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub database_url: String,
    pub server_addr: String,
    pub log_level: String,
    pub log_dir: String,
    pub environment: Environment,
}

impl AppConfig {
    pub fn from_env() -> Result<Self> {
        let env_str = env::var("APP_ENV").unwrap_or_else(|_| "dev".to_string());
        let environment = match env_str.to_lowercase().as_str() {
            "production" | "prod" => Environment::Production,
            _ => Environment::Development,
        };

        let default_log_level = if environment.is_production() {
            "info"
        } else {
            "debug"
        };

        let default_log_dir = if environment.is_production() {
            "./logs"
        } else {
            ""
        };

        Ok(Self {
            database_url: env::var("DATABASE_URL")?,
            server_addr: env::var("SERVER_ADDR").unwrap_or_else(|_| "0.0.0.0:3000".to_string()),
            log_level: env::var("LOG_LEVEL").unwrap_or_else(|_| default_log_level.to_string()),
            log_dir: env::var("LOG_DIR").unwrap_or_else(|_| default_log_dir.to_string()),
            environment,
        })
    }
}