#[allow(dead_code)]
#[derive(serde::Deserialize)]
pub struct Settings {
    pub app_port: u16,
    pub database: DatabaseSettings,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub db_name: String,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    // Setup config reader. 
    let settings = config::Config::builder()
        // Add config path at hard-coded config location. 
        .add_source(
            config::File::new("configuration.yaml", config::FileFormat::Yaml)
        )
        .build()?;
    //Try to convert config to application config type. 
    settings.try_deserialize::<Settings>()
}
