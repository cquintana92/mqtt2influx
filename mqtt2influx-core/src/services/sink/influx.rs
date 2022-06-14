use super::EventSink;
use crate::Event;
use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use influxdb::Client as InfluxClient;
use influxdb::InfluxDbWriteable;
use tokio_compat_02::FutureExt;

pub const READINGS_TABLE: &str = "readings";

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
struct InfluxDbEvent {
    time: DateTime<Utc>,
    #[influxdb(tag)]
    device_name: String,
    temperature: f32,
    humidity: f32,
    battery: u8,
}

impl From<Event> for InfluxDbEvent {
    fn from(e: Event) -> Self {
        Self {
            time: Utc::now(),
            device_name: e.device_name,
            temperature: e.temperature,
            humidity: e.humidity,
            battery: e.battery,
        }
    }
}

impl InfluxDbSink {
    pub async fn new(params: InfluxDbConnectionParameters<'_>) -> Result<Self> {
        let client = Self::create_client(params);
        let (build_type, version) = client.ping().compat().await.context("Error checking connection to InfluxDB")?;
        info!(
            "Successfully connected to InfluxDB [build_type={}] [version={}]",
            build_type, version
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
        let e = InfluxDbEvent::from(event);

        if let Err(e) = self.client.query(&e.into_query(READINGS_TABLE)).compat().await {
            error!("Error sending temperature event to InfluxDb: {}", e.to_string());
        } else {
            info!("Event stored into InfluxDb");
        }
        Ok(())
    }
}
