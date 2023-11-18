mod jni;

pub use jni::attach_current_thread;
pub use jni::init;
pub use jni::nick_name::NickName;

use self::jni::global_ctx;
use self::jni::global_jvm;

extern "C" {
    fn android_get_device_api_level() -> i32;
}
pub fn get_device_api_level() -> i32 {
    unsafe { android_get_device_api_level() }
}

pub fn finish() -> crate::Result<()> {
    let mut jni_env = global_jvm().get_env()?;

    if let Err(e) = jni_env.call_method(global_ctx(), "finishAndRemoveTask", "()V;", &[]) {
        println!("Failed to call finishAndRemoveTask method: {:?}", e);
    }

    Ok(())
}
