use secrecy::{ExposeSecret, Secret};

#[allow(dead_code)]
#[derive(serde::Deserialize)]
pub struct Settings {
    pub app: AppSettings,
    pub database: DatabaseSettings,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: Secret<String>,
    pub password: Secret<String>,
    pub port: u16,
    pub host: String,
    pub name: String,
}

#[derive(serde::Deserialize)]
pub struct AppSettings {
    pub host: String,
    pub port: String,
}

impl DatabaseSettings {
    pub fn get_connection_string(&self) -> Secret<String> {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username.expose_secret(),
            self.password.expose_secret(),
            self.host,
            self.port,
            self.name
        )
        .into()
    }

    pub fn get_connection_string_no_db(&self) -> Secret<String> {
        format!(
            "postgres://{}:{}@{}:{}",
            self.username.expose_secret(),
            self.password.expose_secret(),
            self.host,
            self.port
        )
        .into()
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
