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

    println!("start");

    let api_level = get_device_api_level().unwrap();
    println!("{api_level}");

    if let Ok(device_name) = nick_name::NickName::new() {
        println!("{:?}", device_name.get());
    };

    nick_name::finish().unwrap();

    println!("__finish__");
}
