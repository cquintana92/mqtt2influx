use super::EventSink;
use crate::Event;
use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use influxdb::Client as InfluxClient;
use influxdb::InfluxDbWriteable;
use tokio_compat_02::FutureExt;

const TEMPERATURE_TABLE: &str = "temperature";
const HUMIDITY_TABLE: &str = "humidity";
const BATTERY_TABLE: &str = "battery";

pub struct InfluxDbConnectionParameters<'a> {
    pub server: &'a str,
    pub db: &'a str,
    pub credentials: Option<InfluxDbCredentials<'a>>,
}

pub struct InfluxDbCredentials<'a> {
    pub username: &'a str,
    pub password: &'a str,
}

pub struct InfluxDbSink {
    client: InfluxClient,
}

#[derive(Debug, InfluxDbWriteable)]
struct InfluxDbTemperatureEvent {
    time: DateTime<Utc>,
    device_name: String,
    temperature: f32,
}
#[derive(InfluxDbWriteable)]
struct InfluxDbHumidityEvent {
    time: DateTime<Utc>,
    device_name: String,
    humidity: f32,
}

#[derive(InfluxDbWriteable)]
struct InfluxDbBatteryEvent {
    time: DateTime<Utc>,
    device_name: String,
    battery: u8,
}

impl InfluxDbSink {
    pub async fn new(params: InfluxDbConnectionParameters<'_>) -> Result<Self> {
        let client = Self::create_client(params);
        let (build_type, version) = client.ping().compat().await.context("Error checking connection to InfluxDB")?;
        tracing::info!(
            "Successfully connected to InfluxDB [build_type={}] [version={}]",
            build_type,
            version
        );
        Ok(Self { client })
    }

    fn create_client(params: InfluxDbConnectionParameters) -> InfluxClient {
        let mut client = InfluxClient::new(params.server, params.db);
        if let Some(credentials) = params.credentials {
            client = client.with_auth(credentials.username, credentials.password);
        }
        client
    }
}

#[async_trait::async_trait]
impl EventSink for InfluxDbSink {
    async fn sink(&self, event: Event) -> Result<()> {
        let now = chrono::Utc::now();
        let temperature = InfluxDbTemperatureEvent {
            time: now,
            device_name: event.device_name.clone(),
            temperature: event.temperature,
        };
        let humidity = InfluxDbHumidityEvent {
            time: now,
            device_name: event.device_name.clone(),
            humidity: event.humidity,
        };
        let battery = InfluxDbBatteryEvent {
            time: now,
            device_name: event.device_name.clone(),
            battery: event.battery,
        };

        let mut has_failed = false;
        if let Err(e) = self.client.query(&temperature.into_query(TEMPERATURE_TABLE)).compat().await {
            tracing::error!("Error sending temperature event to InfluxDb: {}", e.to_string());
            has_failed = true;
        }
        if let Err(e) = self.client.query(&humidity.into_query(HUMIDITY_TABLE)).compat().await {
            tracing::error!("Error sending humidity event to InfluxDb: {}", e.to_string());
            has_failed = true;
        }
        if let Err(e) = self.client.query(&battery.into_query(BATTERY_TABLE)).compat().await {
            tracing::error!("Error sending battery event to InfluxDb: {}", e.to_string());
            has_failed = true;
        }

        if has_failed {
            Err(anyhow::anyhow!("There was an error sending the events to InfluxDb"))
        } else {
            tracing::info!("Event stored into InfluxDb");
            Ok(())
        }
    }
}
