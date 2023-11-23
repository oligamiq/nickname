// https://docs.rs/cacao/latest/src/cacao/filesystem/manager.rs.html#23

use std::error::Error;
use std::ffi::CStr;
use std::fmt::{Debug, Formatter};
use std::sync::{Arc, RwLock};

use super::id;
use objc::rc::Id;
use objc::runtime::{AnyObject, NSObject, Object};
use objc::{class, msg_send, msg_send_id, ClassType};

#[repr(transparent)]
#[derive(Clone)]
pub struct NickName(pub Arc<RwLock<id>>);

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
            // Create an instance of the UIDevice class
            msg_send![class!(UIDevice), alloc]
        }))))
    }

    pub fn get(&self) -> crate::Result<String> {
        let device = self.0.read().unwrap();

        // Get the name property
        let current: id = unsafe { msg_send![&**device, name] };

        // Convert the Objective-C string to a Rust string
        let name_cstr: &CStr = unsafe { CStr::from_ptr(msg_send![current, UTF8String]) };

        // Convert &CStr to Rust string
        let name_str = name_cstr.to_str().unwrap();

        Ok(name_str.to_string())
    }
}
