use std::ffi::CStr;

use crate::macos::util::class_id;

pub mod nick_name;

// https://qiita.com/yohei_takada201/items/eaf28d33f29384b6e8b4
/// print all classes for debug
pub fn preview_all_classes() {
    // count = objc_util.objc_getClassList(None, 0)
    // print( '%d classes found:' % count )

    let count = objc::runtime::AnyClass::classes_count();
    println!("classes_count: {:?}", count);

    // classes = (objc_util.c_void_p * count)()
    // count = objc_util.objc_getClassList(classes, count)
    let mut classes: Vec<class_id> = Vec::with_capacity(count);
    let count = unsafe { objc_sys::objc_getClassList(classes.as_mut_ptr(), count as i32) };
    unsafe { classes.set_len(count as usize) };

    println!("classes_count: {:?}", count);

    let class_names = classes
        .iter()
        .map(|class| {
            let name = unsafe { CStr::from_ptr(objc_sys::class_getName(*class)) };
            name.to_string_lossy().into_owned()
        })
        .collect::<Vec<_>>();

    println!("classes: {:?}", class_names);
}
