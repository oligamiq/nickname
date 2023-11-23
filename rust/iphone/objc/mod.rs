use objc::runtime;

pub mod nick_name;

use objc::runtime::NSObject;
use objc::Encode;

#[allow(non_camel_case_types)]
pub type id = *const NSObject;
