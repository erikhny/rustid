use std::collections::HashMap;
use serde::Deserialize;
use crate::oauth::client::Client;
use crate::oauth::client::deserialize_clients;

#[derive(Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
    #[serde(deserialize_with = "deserialize_clients")]
    pub clients: HashMap<String, Client>,
}

#[derive(Deserialize, Clone)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let settings = config::Config::builder()
        .add_source(
            config::File::new("configuration.yaml", config::FileFormat::Yaml)
        )
        .add_source(
            config::File::new("clients.yaml", config::FileFormat::Yaml)
        )
        .build()?;
    settings.try_deserialize::<Settings>()
}