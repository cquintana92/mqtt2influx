use thiserror::Error;

pub mod executor;
pub mod services;
pub mod types;
pub mod utils;

pub use executor::*;
pub use services::*;
pub use types::*;

pub use anyhow;
pub use async_trait;
pub use chrono;
pub use tokio_compat_02;

#[derive(Clone, Debug, Error)]
pub enum AppError {
    #[error("Mqtt error: {0}")]
    Mqtt(String),
    #[error("Server error: {0}")]
    Server(String),
    #[error("Unknown error: {0}")]
    Unknown(String),
}
