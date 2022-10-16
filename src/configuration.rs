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
    pub port: String
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
    // Setup config reader.
    let settings = config::Config::builder()
        // Add config path at hard-coded config location.
        .add_source(config::File::new(
            "configuration.yaml",
            config::FileFormat::Yaml,
        ))
        .build()?;
    //Try to convert config to application config type.
    settings.try_deserialize::<Settings>()
}
