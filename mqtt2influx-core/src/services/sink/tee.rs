use super::EventSink;
use crate::Event;
use anyhow::Result;
use std::sync::Arc;

pub struct SinkTee<L, R>
where
    L: EventSink,
    R: EventSink,
{
    left: Arc<L>,
    right: Arc<R>,
}

impl<L, R> SinkTee<L, R>
where
    L: EventSink,
    R: EventSink,
{
    pub fn new(left: Arc<L>, right: Arc<R>) -> Self {
        Self { left, right }
    }
}

#[async_trait::async_trait]
impl<L, R> EventSink for SinkTee<L, R>
where
    L: EventSink,
    R: EventSink,
{
    async fn sink(&self, event: Event) -> Result<()> {
        let left = self.left.sink(event.clone());
        let right = self.right.sink(event);
        let r = tokio::join!(left, right);
        match r {
            (Ok(_), Ok(_)) => Ok(()),
            (Err(el), _) => Err(el),
            (_, Err(er)) => Err(er),
        }
    }
}
