cfg_if! {
    if #[cfg(target_os = "android")] {
        mod android;
        pub use android::*;
        extern "C" {
            fn android_get_device_api_level() -> i32;
        }
        pub fn get_device_api_level() -> i32 {
            unsafe { android_get_device_api_level() }
        }
    } else if #[cfg(any(unix, target_os = "redox"))] {
        mod linux;
        pub use linux::*;
    } else if #[cfg(target_os = "windows")] {
        mod windows;
        pub use windows::*;
    } else {
        compile_error!("Unsupported target OS! Create an issue: https://github.com/svartalf/hostname/issues/new");
    }
}

use std::{result, time::Duration};

pub type Result<T> = result::Result<T, Error>;

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

impl Into<Error> for std::io::Error {
    fn into(self) -> Error {
        match self.kind() {
            std::io::ErrorKind::PermissionDenied => Error::PermissionDenied,
            _ => Error::Other(Box::new(self)),
        }
    }
}