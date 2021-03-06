use config::{Config as CConfig, ConfigError, Environment, File};
use mqtt2influx_core::{InfluxDbConnectionParameters, InfluxDbCredentials, Subscription};
use std::collections::HashMap;

const DEFAULT_FILE_NAME: &str = "mqtt2influx.toml";
const DEFAULT_PORT: u16 = 3333;

#[cfg(debug_assertions)]
fn default_log_level() -> String {
    "debug".to_string()
}

#[cfg(not(debug_assertions))]
fn default_log_level() -> String {
    "info".to_string()
}

fn default_port() -> u16 {
    DEFAULT_PORT
}

fn default_client_id() -> String {
    "mqtt2influx-client".to_string()
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Connection {
    pub host: String,
    pub port: u16,
    pub username: Option<String>,
    pub password: Option<String>,
}

impl Connection {
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.host.is_empty() {
            return Err(ConfigError::Message("connection.host cannot be empty".to_string()));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct InfluxDbConnection {
    pub server: String,
    pub database: String,
    pub username: Option<String>,
    pub password: Option<String>,
}

impl InfluxDbConnection {
    pub fn as_connection_parameters(&self) -> InfluxDbConnectionParameters {
        let credentials = match (&self.username, &self.password) {
            (Some(username), Some(password)) => Some(InfluxDbCredentials { username, password }),
            _ => None,
        };
        InfluxDbConnectionParameters {
            server: &self.server,
            db: &self.database,
            credentials,
        }
    }

    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.server.is_empty() {
            return Err(ConfigError::Message("influxdb.server cannot be empty".to_string()));
        }

        if self.database.is_empty() {
            return Err(ConfigError::Message("influxdb.server cannot be empty".to_string()));
        }

        if (self.username.is_some() && self.password.is_none()) || (self.username.is_none() && self.password.is_some()) {
            return Err(ConfigError::Message(
                "Either both or none influxdb.username and influxdb.password must be defined".to_string(),
            ));
        }

        Ok(())
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Config {
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default = "default_log_level")]
    pub log_level: String,
    #[serde(default = "default_client_id")]
    pub client_id: String,
    pub mqtt: Connection,
    pub subscriptions: HashMap<String, Subscription>,
    pub influx: InfluxDbConnection,
}

impl Config {
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.subscriptions.is_empty() {
            return Err(ConfigError::Message("Subscription list cannot be empty".to_string()));
        }
        self.mqtt.validate()?;
        self.influx.validate()?;
        Ok(())
    }

    pub fn subscriptions(&self) -> Vec<Subscription> {
        self.subscriptions.values().cloned().collect()
    }
}

pub fn load(path: Option<&str>) -> Result<Config, ConfigError> {
    dotenv::dotenv().ok();
    let mut c = CConfig::new();
    c.merge(Environment::new().separator("__"))?;

    match path {
        Some(p) => c.merge(File::with_name(p).required(true))?,
        None => c.merge(File::with_name(DEFAULT_FILE_NAME).required(false))?,
    };

    let parsed: Config = c.try_into()?;
    parsed.validate()?;
    Ok(parsed)
}
