use std::{result, time::Duration};
#[cfg(target_os = "android")]
mod android;
#[cfg(target_os = "android")]
pub use crate::android::init;
#[cfg(target_os = "android")]
pub use android::UserDeviceName;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Permission denied")]
    PermissionDenied,

    #[error("The operation is not supported: {}", _0)]
    NotSupported(String),

    #[error("Timed out after {:?}", _0)]
    TimedOut(Duration),

    #[error("Runtime Error: {}", _0)]
    RuntimeError(String),

    #[error("{}", _0)]
    Other(Box<dyn std::error::Error + Send + Sync>),
}

pub type Result<T> = result::Result<T, Error>;
