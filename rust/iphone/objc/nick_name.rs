// https://docs.rs/cacao/latest/src/cacao/filesystem/manager.rs.html#23

use std::error::Error;
use std::ffi::CStr;
use std::fmt::{Debug, Formatter};
use std::sync::{Arc, RwLock};

use objc::rc::Id;
use objc::runtime::NSObject;
use objc::{class, msg_send, msg_send_id, ClassType};

#[derive(Clone)]
pub struct NickName(pub Arc<RwLock<Id<NSObject>>>);

impl Debug for NickName {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        f.debug_struct("NickName").finish()
    }
}

// https://developer.apple.com/documentation/uikit/uidevice
// https://simlay.net/uikit-sys/master/src/uikit_sys/Users/simlay/projects/uikit-sys/target/x86_64-apple-ios/debug/build/uikit-sys-344536fd54f83e27/out/uikit.rs.html#98727
impl NickName {
    pub fn new() -> crate::Result<Self> {
        Ok(Self(Arc::new(RwLock::new(unsafe {
            msg_send_id![class!(UIDevice), new]
        }))))
    }

    pub fn get(&self) -> crate::Result<String> {
        let device = self.0.read().unwrap();
        let name: *const std::os::raw::c_char = unsafe { msg_send![&**device, name] };
        let c_str = unsafe { CStr::from_ptr(name) };
        let str_slice: &str = c_str.to_str().unwrap();
        Ok(str_slice.to_string())
    }
}
