mod jni;

use ::jni::JNIEnv;
#[cfg(target_os = "android")]
pub use jni::objects::UserDeviceName;

pub fn init(env: &JNIEnv) -> crate::Result<()> {
    self::jni::init(env)?;
    Ok(())
}
