use android_activity::AndroidApp;
use jni::{sys::JNIInvokeInterface_, JavaVM};
use libc::c_void;
use log::info;
use ndk::trace;

#[no_mangle]
fn android_main(app: AndroidApp) {
    android_logger::init_once(android_logger::Config::default());

    let _trace;
    if trace::is_trace_enabled() {
        _trace = trace::Section::new("ndk-rs example main").unwrap();
    }

    info!("hello world");
    println!("hello world");

    let raw_ptr: *mut c_void = app.vm_as_ptr();

    let vm = unsafe { JavaVM::from_raw(raw_ptr as *mut *const JNIInvokeInterface_) }.unwrap();

    user_device_name::init(&vm.get_env().unwrap()).unwrap();

    let device_name = user_device_name::UserDeviceName::new()
        .unwrap_or_else(|e| panic!("Failed to get device name: {}", e))
        .get()
        .unwrap_or_else(|e| panic!("Failed to get device name: {}", e));
    println!("{device_name}");
}
