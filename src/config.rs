use std::io::Stderr;

use serde::Deserialize;
use config::{Config as RawConfig, ConfigError, Environment};

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
  pub database: Database,
  pub server: Server,
  pub auth: Auth,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Database {
  pub url: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Server {
  #[serde(default = "default_server_host")]
  pub host: String,
  #[serde(default = "default_server_port")]
  pub port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Auth { 
  pub jwt_secret: String,
  #[serde(default = "default_expiration_seconds")]
  pub expiration_seconds: i64,
}

fn default_server_host() -> String {
  "127.0.0.1".into()
}

fn default_server_port() -> u16 {
  8000
}

fn default_expiration_seconds() -> i64 {
  3600
}

impl Config {
  pub fn load() -> Result<Self, ConfigError> {
    // Load .env.local (if exists, as override)
    dotenvy::from_filename(".env.local").ok();

    // Then load .env (fallback)
    dotenvy::dotenv().map_err(|err| {
      ConfigError::Message(format!(".env file load error: {}", err))
    })?;

    // Get DATABASE_URL from env manually
    let db_url = std::env::var("DATABASE_URL")
      .map_err(|_| ConfigError::Message("DATABASE_URL must be set".into()))?;

    // Build config with override
    let builder = RawConfig::builder()
      .add_source(Environment::default().separator("__"))
      .set_override("database.url", db_url)?; // 👈 manual override here

    let config: Config = builder.build()?.try_deserialize()?;
    config.validate()?;
    Ok(config)
  }

  fn validate(&self) -> Result<(), ConfigError> {
    if self.database.url.trim().is_empty() {
      return Err(ConfigError::Message("DATABASE_URL is required".into()));
    }

    if self.auth.jwt_secret.len() < 32 {
      return Err(ConfigError::Message(
        "AUTH__JWT_SECRET must be at least 32 characters long".into(),
      ));
    }

    Ok(())
  }
}