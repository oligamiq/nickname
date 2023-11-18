use android_activity::AndroidApp;
use ndk::trace;
use nick_name::get_device_api_level;

#[no_mangle]
fn android_main(_app: AndroidApp) {
    android_logger::init_once(android_logger::Config::default());

    let _trace;
    if trace::is_trace_enabled() {
        _trace = trace::Section::new("ndk-rs example main").unwrap();
    }

    let device_name = nick_name::NickName::new().unwrap().get().unwrap();
    println!("{device_name}");

    let api_level = get_device_api_level();
    println!("{api_level}");

    nick_name::finish().unwrap();
}
