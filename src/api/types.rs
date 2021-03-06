use mqtt2influx_core::anyhow::Result;
use mqtt2influx_core::chrono::Utc;
use mqtt2influx_core::{async_trait, Event, EventSink};
use std::collections::HashMap;
use tokio::sync::RwLock;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct APIEvent {
    name: String,
    temperature: f32,
    updated_at: i64,
}

pub struct ApiState {
    contents: RwLock<HashMap<String, APIEvent>>,
}

impl ApiState {
    pub async fn values(&self) -> Vec<APIEvent> {
        self.contents.read().await.values().cloned().collect()
    }
}

impl Default for ApiState {
    fn default() -> Self {
        Self {
            contents: RwLock::new(HashMap::new()),
        }
    }
}

#[async_trait::async_trait]
impl EventSink for ApiState {
    async fn sink(&self, event: Event) -> Result<()> {
        let mut contents = self.contents.write().await;
        let api_event = APIEvent {
            name: event.device_name.clone(),
            temperature: event.temperature,
            updated_at: Utc::now().naive_utc().timestamp(),
        };
        contents.insert(event.device_name, api_event);
        Ok(())
    }
}
