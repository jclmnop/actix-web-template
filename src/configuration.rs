use anyhow::anyhow;
use secrecy::{ExposeSecret, Secret};
use std::path::Path;

const CONFIG_DIR: &str = "config";
const BASE_CONFIG_FILE: &str = "base.yml";

type ConfigFile = config::File<config::FileSourceFile, config::FileFormat>;

#[derive(Clone)]
pub struct HmacSecret(pub Secret<String>);

/// Settings for the App
#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub app: AppSettings,
}

impl Settings {
    pub fn get_config() -> Result<Self, config::ConfigError> {
        let base_path = std::env::current_dir()
            .map_err(|_| {
                config::ConfigError::NotFound("Current dir not found".into())
            })?;
        let config_directory = base_path.join(CONFIG_DIR);

        let environment = Environment::get_env();
        let base_config_file = Self::base_config_file(&config_directory);
        let env_config_file =
            Self::env_config_file(&config_directory, &environment);

        let settings = config::Config::builder()
            .add_source(base_config_file)
            .add_source(env_config_file)
            .build()?;

        settings.try_deserialize::<Settings>()
    }

    pub fn get_address(&self) -> String {
        format!("{}:{}", self.app.host, self.app.port)
    }

    fn base_config_file(dir: &Path) -> ConfigFile {
        config::File::from(dir.join(BASE_CONFIG_FILE))
    }

    fn env_config_file(dir: &Path, env: &Environment) -> ConfigFile {
        config::File::from(dir.join(env.as_filename()))
    }
}

#[derive(serde::Deserialize, Clone)]
pub struct AppSettings {
    pub host: String,
    pub port: u16,
    pub hmac_secret: Secret<String>,
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

pub enum Environment {
    Local,
    Prod,
}

impl Environment {
    pub fn get_env() -> Self {
        std::env::var("APP_ENVIRONMENT")
            .unwrap_or_else(|_| "local".into())
            .try_into()
            .expect("Failed to parse APP_ENVIRONMENT variable")
    }

    pub fn as_filename(&self) -> String {
        format!("{}.yml", self.as_str())
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Prod => "prod",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "prod" | "production" => Ok(Self::Prod),
            other => Err(anyhow!(
                "{other} is not a supported environment. \
                Supported environments: [`local`, `prod`, `production`]"
            )),
        }
    }
}
