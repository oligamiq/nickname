use std::sync::OnceLock;

use jni::{JNIEnv, JavaVM};

pub mod objects;

static GLOBAL_JVM: OnceLock<JavaVM> = OnceLock::new();

pub fn init(env: &JNIEnv) -> crate::Result<()> {
    if let Ok(()) = GLOBAL_JVM.set(env.get_java_vm()?) {};
    Ok(())
}
// pub fn init() -> crate::Result<()> {
//     let env = jni::JavaVM::new().unwrap();
//     if let Ok(()) = GLOBAL_JVM.set(env.get_java_vm()?) {};
//     Ok(())
// }

pub fn global_jvm() -> &'static JavaVM {
    GLOBAL_JVM
        .get()
        .expect("JVM has not been initialized. Please initialize it with init().")
}

impl From<::jni::errors::Error> for crate::Error {
    fn from(err: ::jni::errors::Error) -> Self {
        Self::Other(Box::new(err))
    }
}
