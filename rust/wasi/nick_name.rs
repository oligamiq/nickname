// https://github.com/svartalf/hostname/blob/master/src/nix.rs

use std::ffi::CStr;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::io::ErrorKind;

use crate::Error;
const _POSIX_HOST_NAME_MAX: libc::c_long = 255;

pub struct NickName {}

impl Debug for NickName {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        f.debug_struct("NickName").finish()
    }
}

impl NickName {
    pub fn new() -> crate::Result<Self> {
        Ok(Self {})
    }

    pub fn get(&self) -> crate::Result<String> {
        Err(crate::Error::NotSupported(
            "wasi is not support hostname now".into(),
        ))
    }
}
