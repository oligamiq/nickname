use objc::runtime;

pub mod nick_name;

#[allow(non_camel_case_types)]
pub type id = *const objc::runtime::NSObject;
