use crate::domain::ListSubscriberEmail;
use secrecy::{ExposeSecret, Secret};
use sqlx::postgres::{PgConnectOptions, PgSslMode};
use sqlx::ConnectOptions;
use std::time::Duration;

#[allow(dead_code)]
#[derive(serde::Deserialize)]
pub struct Settings {
    pub app: AppSettings,
    pub database: DatabaseSettings,
    pub email_client: EmailClientSettings,
}

#[derive(serde::Deserialize)]
pub struct EmailClientSettings {
    pub base_url: String,
    pub sender_string: String,
    pub auth_token: Secret<String>,
    pub timeout_secs: u64,
}

impl EmailClientSettings {
    pub fn sender(&self) -> Result<ListSubscriberEmail, String> {
        ListSubscriberEmail::try_from(self.sender_string.clone())
    }
    pub fn timeout(&self) -> Duration {
        Duration::from_secs(self.timeout_secs)
    }
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: Secret<String>,
    pub password: Secret<String>,
    pub port: String,
    pub host: String,
    pub name: String,
    pub require_ssl: bool,
}

#[derive(serde::Deserialize)]
pub struct AppSettings {
    pub host: String,
    pub port: String,
    pub base_url: String,
}

impl DatabaseSettings {
    pub fn with_db(&self) -> PgConnectOptions {
        let mut out = self.without_db().database(&self.name);
        out.log_statements(tracing::log::LevelFilter::Trace);
        out
    }

    pub fn without_db(&self) -> PgConnectOptions {
        let ssl_mode = if self.require_ssl {
            PgSslMode::Require
        } else {
            PgSslMode::Prefer
        };
        PgConnectOptions::new()
            .host(&self.host)
            .port(self.port.parse().unwrap())
            .username(self.username.expose_secret())
            .password(self.password.expose_secret())
            .ssl_mode(ssl_mode)
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    // Parse Environment
    let run_type: RunType = std::env::var("RUN_TYPE")
        .unwrap_or_else(|_| "dev".into())
        .try_into()
        .expect("Failed to parse run type env var");
    let env_conf_fname = format!("config/{}.yaml", run_type.as_str());
    // Setup config reader.
    let settings = config::Config::builder()
        // Add config path at hard-coded config location.
        .add_source(config::File::new(
            "config/base.yaml",
            config::FileFormat::Yaml,
        ))
        .add_source(config::File::new(&env_conf_fname, config::FileFormat::Yaml))
        .add_source(
            config::Environment::with_prefix("APP")
                .prefix_separator("_")
                .separator("__"),
        )
        .build()?;
    //Try to convert config to application config type.
    settings.try_deserialize::<Settings>()
}

enum RunType {
    Dev,
    Prod,
}

impl RunType {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Dev => "dev",
            Self::Prod => "prod",
        }
    }
}

impl TryFrom<String> for RunType {
    type Error = &'static str;
    fn try_from(val: String) -> Result<Self, Self::Error> {
        match &val[..] {
            "dev" => Ok(Self::Dev),
            "prod" => Ok(Self::Prod),
            _ => Err("Failed to parse run type"),
        }
    }
}
