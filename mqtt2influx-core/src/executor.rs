use crate::{EventSink, EventSource};
use anyhow::Result;

pub struct Executor;

impl Executor {
    pub async fn run<Source, Sink>(source: Source, sink: &Sink) -> Result<()>
    where
        Source: EventSource,
        Sink: EventSink,
    {
        info!("Starting EventSource");
        let mut rx = source.start().await?;

        info!("Receiving events");
        while let Some(event) = rx.recv().await {
            info!("Event received: {:?}", event);
            if let Err(e) = sink.sink(event).await {
                error!("Error sinking event: {}", e);
            }
        }
        Ok(())
    }
}
