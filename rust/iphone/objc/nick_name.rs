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
        println!("NickName::new()");

        Ok(Self(Arc::new(RwLock::new(unsafe {
            msg_send![class!(UIDevice222), alloc]
        }))))
    }

    pub fn get(&self) -> crate::Result<String> {
        println!("NickName::get()");

        let device = self.0.read().unwrap();

        println!("device: {:?}", device);

        let any_class = unsafe { &**device }.class();

        let obj: &NSObject = unsafe { &**device };

        println!("obj: {:?}", obj);

        // let cur: NSObject = unsafe { *obj.ivar_ptr("current") };

        // println!("cur: {:?}", cur);

        println!("any_class: {:?}", any_class);

        let current: Id<NSObject> = unsafe { msg_send_id![&**device, current] };

        println!("current: {:?}", current);

        let name: *const std::os::raw::c_char = unsafe { msg_send![&**device, name] };

        println!("name: {:?}", name);

        let c_str = unsafe { CStr::from_ptr(name) };

        println!("c_str: {:?}", c_str);

        let str_slice: &str = c_str.to_str().unwrap();

        println!("str_slice: {:?}", str_slice);

        Ok(str_slice.to_string())
    }
}
