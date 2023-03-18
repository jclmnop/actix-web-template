use anyhow::anyhow;
use secrecy::{ExposeSecret, Secret};
use sqlx::postgres::{PgConnectOptions, PgSslMode};
use sqlx::ConnectOptions;
use std::io::Read;
use std::path::Path;

const CONFIG_DIR: &str = "config";
const BASE_CONFIG_FILE: &str = "base.yml";

type ConfigFile = config::File<config::FileSourceString, config::FileFormat>;

#[derive(Clone)]
pub struct HmacSecret(pub Secret<String>);

/// Settings for the App
#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub app: AppSettings,
    pub redis_uri: Secret<String>,
}

// TODO: Use shellexpand crate?
impl Settings {
    pub fn get_config() -> Result<Self, config::ConfigError> {
        let base_path = std::env::current_dir().map_err(|_| {
            config::ConfigError::NotFound("Current dir not found".into())
        })?;
        let config_directory = base_path.join(CONFIG_DIR);

        let environment = Environment::get_env();
        if environment == Environment::Local {
            dotenvy::dotenv().expect(".env not found");
        }
        let base_config_file = Self::base_config_file(&config_directory);
        let env_specific_config_file = Self::environment_specific_config_file(
            &config_directory,
            &environment,
        );

        let settings = config::Config::builder()
            .add_source(base_config_file)
            .add_source(env_specific_config_file)
            // Overwrite settings using environment variables
            .add_source(Self::env_vars())
            .build()?;

        settings.try_deserialize::<Settings>()
    }

    pub fn get_address(&self) -> String {
        format!("{}:{}", self.app.host, self.app.port)
    }

    fn base_config_file(dir: &Path) -> ConfigFile {
        Self::load_config_file(&dir.join(BASE_CONFIG_FILE), None)
    }

    fn environment_specific_config_file(
        dir: &Path,
        env: &Environment,
    ) -> ConfigFile {
        Self::load_config_file(&dir.join(env.as_filename()), None)
    }

    /// Loads the config file and expands any environment $VARIABLES
    fn load_config_file(
        dir: &Path,
        format: Option<config::FileFormat>,
    ) -> ConfigFile {
        let format = format.unwrap_or(config::FileFormat::Yaml);
        let mut file =
            std::fs::File::open(dir).expect("Error reading config file");

        // Expand environment values
        let mut conf_str = String::new();
        file.read_to_string(&mut conf_str)
            .expect("Error reading config file to string");
        conf_str = shellexpand::env(&conf_str).unwrap().into();

        config::File::from_str(&conf_str, format)
    }

    /// Used to overwrite config settings with environment variables, handy for
    /// tweaking settings without rebuilding container.
    ///
    /// e.g. `$OVERWRITE_APP-HMAC_SECRET` -> `Settings.app.hmac_secret`
    fn env_vars() -> config::Environment {
        config::Environment::with_prefix("OVERWRITE")
            .prefix_separator("_")
            .separator("-")
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
    pub require_ssl: bool,
}

impl DatabaseSettings {
    /// Connection string for database
    pub fn with_db(&self) -> PgConnectOptions {
        let mut db_options = self.without_db().database(&self.database_name);
        db_options.log_statements(tracing::log::LevelFilter::Trace);
        db_options
    }

    /// Connection string for top level Postgres instance
    pub fn without_db(&self) -> PgConnectOptions {
        let ssl_mode = match self.require_ssl {
            true => PgSslMode::Require,
            false => PgSslMode::Prefer,
        };

        PgConnectOptions::new()
            .host(&self.host)
            .port(self.port)
            .username(&self.username)
            .password(self.password.expose_secret())
            .ssl_mode(ssl_mode)
    }
}

#[derive(PartialEq)]
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
