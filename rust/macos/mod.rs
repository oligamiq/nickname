mod objc;
pub use self::objc::nick_name::NickName;

mod util {
    use objc::runtime::NSObject;

    #[allow(non_camel_case_types)]
    pub(crate) type id = *const NSObject;
}
