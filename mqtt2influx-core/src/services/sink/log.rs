use super::EventSink;
use crate::Event;
use anyhow::Result;

pub struct LogSink;

#[async_trait::async_trait]
impl EventSink for LogSink {
    async fn sink(&self, event: Event) -> Result<()> {
        info!("Sink event: {:?}", event);
        Ok(())
    }
}
