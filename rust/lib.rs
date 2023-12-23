use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(target_os = "android")] {
        mod android;
        pub use android::*;
    } else if #[cfg(target_os = "ios")] {
        mod iphone;
        pub use iphone::*;
    } else if #[cfg(target_os = "macos")] {
        mod macos;
        pub use macos::*;
    } else if #[cfg(any(unix, target_os = "redox"))] {
        mod linux;
        pub use linux::*;
    } else if #[cfg(target_os = "windows")] {
        mod windows;
        pub use windows::*;
    } else if #[cfg(target_os = "wasi")] {
        mod wasi;
        pub use wasi::*;
        compile_error!("Unsupported target OS! wasi-libc can not supported yet. Create an issue: https://github.com/nziq53/nickname/issues/new");
    } else {
        compile_error!("Unsupported target OS! Create an issue: https://github.com/nziq53/nickname/issues/new");
    }
}

use std::{result, time::Duration};

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Permission denied")]
    PermissionDenied,

    #[error("BluetoothAdapter is null. Maybe bluetooth feature is not supported")]
    BluetoothAdapterNull,

    #[error("API level is too low. Required API level is 5 or higher")]
    ApiLevelTooLow,

    #[error("The operation is not supported: {}", _0)]
    NotSupported(String),

    #[error("Timed out after {:?}", _0)]
    TimedOut(Duration),

    #[error("Runtime Error: {}", _0)]
    RuntimeError(String),

    #[error("OS not supported: {}", _0)]
    OsNotSupported(String),

    #[error("{}", _0)]
    Other(Box<dyn std::error::Error + Send + Sync>),
}

impl From<std::io::Error> for Error {
    fn from(val: std::io::Error) -> Self {
        match val.kind() {
            std::io::ErrorKind::PermissionDenied => Error::PermissionDenied,
            _ => Error::Other(Box::new(val)),
        }
    }
}
