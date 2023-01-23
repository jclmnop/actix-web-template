use secrecy::{ExposeSecret, Secret};

const CONFIG_FILE: &str = "config.yaml";
const CONFIG_FORMAT: config::FileFormat = config::FileFormat::Yaml;

/// Settings for the App
#[derive(serde::Deserialize)]
pub struct Settings {
    pub host: String,
    pub application_port: u16,
    pub database: DatabaseSettings,
}

impl Settings {
    pub fn get_config() -> Result<Self, config::ConfigError> {
        let config_file = config::File::new(CONFIG_FILE, CONFIG_FORMAT);
        let settings =
            config::Config::builder().add_source(config_file).build()?;

        settings.try_deserialize::<Settings>()
    }

    pub fn get_address(&self) -> String {
        format!("{}:{}", self.host, self.application_port)
    }
}

/// Settings for the database
#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: Secret<String>,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

impl DatabaseSettings {
    /// Connection string for database
    pub fn connection_string(&self) -> Secret<String> {
        let host = &self.host;
        let port = self.port;
        let username = &self.username;
        let password = &self.password.expose_secret();
        let database_name = &self.database_name;
        Secret::new(format!(
            "postgres://{username}:{password}@{host}:{port}/{database_name}"
        ))
    }

    /// Connection string for top level Postgres instance
    pub fn connection_string_without_db(&self) -> Secret<String> {
        let host = &self.host;
        let port = self.port;
        let username = &self.username;
        let password = &self.password.expose_secret();
        Secret::new(format!("postgres://{username}:{password}@{host}:{port}"))
    }
}
