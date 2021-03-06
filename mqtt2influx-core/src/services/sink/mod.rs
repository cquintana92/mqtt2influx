use crate::types::*;
use anyhow::Result;

pub use influx::*;
pub use log::*;
pub use tee::*;

mod influx;
mod log;
mod tee;

#[async_trait::async_trait]
pub trait EventSink: Send + Sync {
    async fn sink(&self, event: Event) -> Result<()>;
}
