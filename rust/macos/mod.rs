mod obj_c;
pub use self::obj_c::nick_name::NickName;
pub use self::obj_c::preview_all_classes;

mod util {
    use objc::runtime::NSObject;

    #[allow(non_camel_case_types)]
    pub(crate) type id = *const NSObject;

    #[allow(non_camel_case_types)]
    pub(crate) type class_id = *const objc_sys::objc_class;
}
