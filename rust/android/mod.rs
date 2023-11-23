mod jni;

pub use self::jni::attach_current_thread;
pub use self::jni::init;
pub use self::jni::nick_name::NickName;

use self::jni::global_ctx;
use self::jni::global_jvm;

// extern "C" {
//     fn android_get_device_api_level() -> i32;
// }
pub fn get_device_api_level() -> crate::Result<i32> {
    let mut jni_env = global_jvm().get_env()?;

    let obj = jni_env.find_class("android/os/Build$VERSION")?;

    if let Ok(v) = jni_env.get_static_field(obj, "SDK_INT", "I") {
        return Ok(v.i()?);
    }

    // Ok(unsafe { android_get_device_api_level() })
    Ok(0)
}

pub fn finish() -> crate::Result<()> {
    let mut jni_env = global_jvm().get_env()?;

    if get_device_api_level()? < 21 {
        return Err(crate::Error::NotSupported(
            "finishAndRemoveTask is not supported".into(),
        ));
    }

    jni_env.call_method(global_ctx(), "finishAndRemoveTask", "()V", &[])?;

    Ok(())
}

pub fn finish_this_process() -> crate::Result<()> {
    let mut jni_env = global_jvm().get_env()?;

    let obj = jni_env.find_class("android/os/Process")?;

    let pid = jni_env
        .call_static_method(&obj, "myPid", "()I", &[])
        .unwrap()
        .i()
        .unwrap();

    println!("pid: {}", pid);

    if let Err(e) = jni_env.call_static_method(obj, "killProcess", "(I)V", &[pid.into()]) {
        println!("Failed to call finishAndRemoveTask method: {:?}", e);
    }

    Ok(())
}
