use anyhow::Result;
use mqtt2influx_core::services::*;
use mqtt2influx_core::types::*;
use mqtt2influx_core::utils::generate_random_token;
use tokio::sync::mpsc::{channel, Receiver};
use tokio::sync::RwLock;

pub struct MockEventSink {
    pub events: RwLock<Vec<Event>>,
}

impl Default for MockEventSink {
    fn default() -> Self {
        Self {
            events: RwLock::new(Vec::new()),
        }
    }
}

impl MockEventSink {
    pub async fn received(&self) -> Vec<Event> {
        self.events.read().await.clone()
    }
}

#[async_trait::async_trait]
impl EventSink for MockEventSink {
    async fn sink(&self, event: Event) -> Result<()> {
        self.events.write().await.push(event);
        Ok(())
    }
}

pub struct MockEventSource {
    pub events: Vec<Event>,
}

#[async_trait::async_trait]
impl EventSource for MockEventSource {
    async fn start(self) -> Result<Receiver<Event>> {
        let (tx, rx) = channel(10);
        tokio::spawn(async move {
            for event in self.events {
                let _ = tx.send(event).await;
            }
        });
        Ok(rx)
    }
}

pub fn random_event() -> Event {
    Event {
        device_name: generate_random_token(10),
        battery: 1,
        humidity: 2.3,
        temperature: 4.5,
        voltage: 6,
        linkquality: 7,
    }
}
