use std::ffi::OsString;

use config::{Config, ConfigError, FileFormat};

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize)]
pub struct HttpSettings {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize)]
pub struct LogSettings {
    pub directive: String,
    pub log_dir: String,
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize)]
pub struct DatabaseSettings {
    pub host: String,
    pub port: u16,
    pub database_name: String,
    pub app_password: String,
    pub migration_username: String,
    pub migration_password: String,
}

impl DatabaseSettings {
    fn conn_string(&self, username: &str, password: &str) -> String {
        format!(
            "postgresql://{}:{}@{}:{}/{}",
            username, password, self.host, self.port, self.database_name
        )
    }

    pub fn app_conn_string(&self) -> String {
        self.conn_string("app", &self.app_password)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize)]
pub struct Settings {
    pub http: HttpSettings,
    pub log: LogSettings,
    pub database: DatabaseSettings,
}

#[derive(Debug, PartialEq, Eq)]
pub enum EnvironmentError {
    NotUnicode(OsString),
}

const PREFIX: &str = "SHORTEST_URL";
const SETTINGS_FILE_NAME: &str = "settings";

impl Settings {
    pub fn environment() -> Result<String, EnvironmentError> {
        use std::env::{var, VarError};

        match var(format!("{}_ENVIRONMENT", PREFIX)) {
            Ok(env) => Ok(env.trim().to_owned()),
            Err(VarError::NotPresent) => Ok(String::from("")),
            Err(VarError::NotUnicode(s)) => Err(EnvironmentError::NotUnicode(s)),
        }
    }

    pub fn from_env(environemnt: &str) -> Result<Self, ConfigError> {
        let mut config = Config::default();

        let base_settings = format!("{}.yaml", SETTINGS_FILE_NAME);
        config.merge(config::File::new(&base_settings, FileFormat::Yaml).required(false))?;

        if !environemnt.is_empty() {
            let env_settings = format!("{}.{}.yaml", SETTINGS_FILE_NAME, environemnt);
            config.merge(config::File::new(&env_settings, FileFormat::Yaml).required(false))?;
        }

        config.merge(config::Environment::with_prefix(PREFIX).separator("_"))?;

        config.try_into()
    }

    pub fn new() -> Result<Self, ConfigError> {
        let env = Settings::environment().expect("Failed to read enviromnent from env variable.");
        Settings::from_env(&env)
    }
}
