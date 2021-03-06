use crate::{EventSink, EventSource};
use anyhow::Result;

pub struct Executor;

impl Executor {
    pub async fn run<Source, Sink>(source: Source, sink: &Sink) -> Result<()>
    where
        Source: EventSource,
        Sink: EventSink,
    {
        tracing::info!("Starting EventSource");
        let mut rx = source.start().await?;

        tracing::info!("Receiving events");
        while let Some(event) = rx.recv().await {
            tracing::info!("Event received: {:?}", event);
            if let Err(e) = sink.sink(event).await {
                tracing::error!("Error sinking event: {}", e);
            }
        }
        Ok(())
    }
}
