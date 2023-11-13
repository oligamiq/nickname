use jni::objects::{GlobalRef, JString};

use std::fmt::{Debug, Formatter};

use super::global_jvm;

#[derive(Clone)]
pub struct UserDeviceName {
    internal: GlobalRef,
}

impl Debug for UserDeviceName {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        f.debug_struct("UserDeviceName").finish()
    }
}

impl UserDeviceName {
    pub fn new() -> crate::Result<Self> {
        let mut env = global_jvm().get_env()?;

        let obj = env
            .new_object(
                "com/oligami/rust-user-device-name-impl/android/impl/UserDeviceName",
                "()V",
                &[],
            )
            .unwrap();

        let internal = env.new_global_ref(obj)?;

        let user_device_name = Self { internal };

        Ok(user_device_name)
    }

    pub fn get(&mut self) -> crate::Result<String> {
        let mut env = global_jvm().get_env()?;
        let msg = env
            .call_method(
                &self.internal,
                "getUserDeviceName",
                "()Ljava/lang/String;",
                &[],
            )
            .unwrap();
        let msg = msg.l().unwrap();
        let msg = msg.into();
        let msg_str: String = env.get_string(&msg).unwrap().into();
        Ok(msg_str)
    }
}
