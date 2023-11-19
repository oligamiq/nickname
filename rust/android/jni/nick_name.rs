use jni::objects::{JClass, JObject, JValue};

use std::fmt::{Debug, Formatter};

use super::{global_ctx, global_jvm};

pub type Context = JObject<'static>;

pub struct NickName {}

impl Debug for NickName {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        f.debug_struct("NickName").finish()
    }
}

impl NickName {
    pub fn new() -> crate::Result<Self> {
        Ok(Self {})
    }

    fn get_content_resolver(&self) -> crate::Result<JObject> {
        let content_resolver: JObject = global_jvm()
            .get_env()?
            .call_method(
                global_ctx(),
                "getContentResolver",
                "()Landroid/content/ContentResolver;",
                &[],
            )
            .expect("Failed to call getContentResolver method")
            .l()?;

        Ok(content_resolver)
    }

    // fn get_str<S: Into<JNIString>>(&self, str: S) -> crate::Result<JString> {
    //     Ok(global_jvm()
    //         .get_env()?
    //         .new_string(str)
    //         .expect("Failed to create new string"))
    // }

    // fn get_setting_secure(&self) -> crate::Result<JClass> {
    //     Ok(global_jvm()
    //         .get_env()?
    //         .find_class("android/provider/Settings$Secure")?)
    // }

    // fn get_setting_system(&self) -> crate::Result<JClass> {
    //     Ok(global_jvm()
    //         .get_env()?
    //         .find_class("android/provider/Settings$System")?)
    // }

    fn get_setting_global(&self) -> crate::Result<JClass> {
        Ok(global_jvm()
            .get_env()?
            .find_class("android/provider/Settings$Global")?)
    }

    pub fn get(&self) -> crate::Result<String> {
        if 25 > crate::get_device_api_level()? {
            return Err(crate::Error::Other(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "API level 25 or higher is required",
            ))));
        }

        let device_name = self.get_sdk_official()?;

        Ok(device_name)
    }

    // #[cfg(feature = "target-sdk-version-31")]
    // java.lang.SecurityException: Settings key: <bluetooth_name> is only readable to apps with targetSdkVersion lower than or equal to: 31
    // fn get_sdk_31(&self) -> crate::Result<String> {
    //     let content_resolver = self.get_content_resolver()?;
    //     let setting_secure = self.get_setting_secure()?;
    //     let mut jni_env = global_jvm().get_env()?;
    //     let j_str = self.get_str("bluetooth_name")?;

    //     let device_name = global_jvm()
    //         .get_env()?
    //         .call_static_method(
    //             setting_secure,
    //             "getString",
    //             "(Landroid/content/ContentResolver;Ljava/lang/String;)Ljava/lang/String;",
    //             &[JValue::Object(&content_resolver), JValue::Object(&j_str)],
    //         )
    //         .expect("Failed to call getString method")
    //         .l()?
    //         .into();

    //     let device_name: String = jni_env.get_string(&device_name)?.into();

    //     Ok(device_name)
    // }

    // そもそもリファレンスに載ってない
    // fn get_sdk_old(&self) -> crate::Result<String> {
    //     let content_resolver = self.get_content_resolver()?;
    //     let setting_system = self.get_setting_system()?;
    //     let mut jni_env = global_jvm().get_env()?;
    //     let j_str = self.get_str("bluetooth_name")?;

    //     let device_name = global_jvm()
    //         .get_env()?
    //         .call_static_method(
    //             setting_system,
    //             "getString",
    //             "(Landroid/content/ContentResolver;Ljava/lang/String;)Ljava/lang/String;",
    //             &[JValue::Object(&content_resolver), JValue::Object(&j_str)],
    //         )
    //         .expect("Failed to call getString method")
    //         .l()?
    //         .into();

    //     let device_name: String = jni_env.get_string(&device_name)?.into();

    //     Ok(device_name)
    // }

    // そもそもリファレンスに載ってない
    // fn get_sdk_failed(&self) -> crate::Result<String> {
    //     let content_resolver = self.get_content_resolver()?;
    //     let setting_system = self.get_setting_system()?;
    //     let mut jni_env = global_jvm().get_env()?;

    //     let device_name = jni_env
    //         .get_static_field(&setting_system, "DEVICE_NAME", "Ljava/lang/String;")
    //         ?
    //         .l()
    //         ?
    //         .into();

    //     let device_name = global_jvm()
    //         .get_env()
    //         ?
    //         .call_static_method(
    //             setting_system,
    //             "getString",
    //             "(Landroid/content/ContentResolver;Ljava/lang/String;)Ljava/lang/String;",
    //             &[
    //                 JValue::Object(&content_resolver),
    //                 JValue::Object(&device_name),
    //             ],
    //         )
    //         .expect("Failed to call getString method")
    //         .l()
    //         ?
    //         .into();

    //     let device_name: String = jni_env.get_string(&device_name)?.into();

    //     Ok(device_name)
    // }

    // そもそもリファレンスに載ってない
    // fn get_sdk_failed(&self) -> crate::Result<String> {
    //     let content_resolver = self.get_content_resolver()?;
    //     let setting_secure = self.get_setting_secure()?;
    //     let mut jni_env = global_jvm().get_env()?;
    //     let lock_screen_owner_info = jni_env
    //         .get_static_field(
    //             &setting_secure,
    //             "LOCK_SCREEN_OWNER_INFO",
    //             "Ljava/lang/String;",
    //         )
    //         ?
    //         .l()
    //         ?
    //         .into();

    //     let device_name = global_jvm()
    //         .get_env()
    //         ?
    //         .call_static_method(
    //             setting_secure,
    //             "getString",
    //             "(Landroid/content/ContentResolver;Ljava/lang/String;)Ljava/lang/String;",
    //             &[
    //                 JValue::Object(&content_resolver),
    //                 JValue::Object(&lock_screen_owner_info),
    //             ],
    //         )
    //         .expect("Failed to call getString method")
    //         .l()
    //         ?
    //         .into();

    //     let device_name: String = jni_env.get_string(&device_name)?.into();

    //     Ok(device_name)
    // }

    // APIレベル25以降
    fn get_sdk_official(&self) -> crate::Result<String> {
        let content_resolver = self.get_content_resolver()?;
        let setting_global = self.get_setting_global()?;
        let mut jni_env = global_jvm().get_env()?;

        let device_name = jni_env
            .get_static_field(&setting_global, "DEVICE_NAME", "Ljava/lang/String;")?
            .l()?
            .into();

        let device_name = jni_env
            .call_static_method(
                setting_global,
                "getString",
                "(Landroid/content/ContentResolver;Ljava/lang/String;)Ljava/lang/String;",
                &[
                    JValue::Object(&content_resolver),
                    JValue::Object(&device_name),
                ],
            )?
            .l()?
            .into();

        let device_name: String = jni_env.get_string(&device_name)?.into();

        Ok(device_name)
    }
}

// https://stackoverflow.com/questions/16704597/how-do-you-get-the-user-defined-device-name-in-android
// https://medium.com/@pribble88/how-to-get-an-android-device-nickname-4b4700b3068c
