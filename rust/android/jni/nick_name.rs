use jni::{
    objects::{JClass, JObject, JString, JValue},
    strings::JNIString,
};

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

    fn get_str<S: Into<JNIString>>(&self, str: S) -> crate::Result<JString> {
        Ok(global_jvm()
            .get_env()?
            .new_string(str)
            .expect("Failed to create new string"))
    }

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

    /// This fn exchange get method by API level
    /// lowest API level 5
    ///
    /// 5 <= API level <= 17
    /// * Require BLUETOOTH permission
    /// * Require BLUETOOTH_CONNECT permission
    /// * Require bluetooth feature
    /// * Android Emulator don't has bluetooth feature
    ///
    /// 18 <= API level <= 24
    /// * Require BLUETOOTH permission
    /// * Require BLUETOOTH_CONNECT permission
    /// * Require bluetooth feature
    /// * Android Emulator don't has bluetooth feature
    ///
    /// 25 <= API level
    /// * Not require any permission
    pub fn get(&self) -> crate::Result<String> {
        let device_name = match crate::get_device_api_level()? {
            5..=17 => self.get_bluetooth_name_old()?,
            18..=24 => self.get_bluetooth_name_new()?,
            25..=i32::MAX => self.get_sdk_official()?,
            _ => return Err(crate::Error::ApiLevelTooLow),
        };
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

    // lowest API level 25
    pub fn get_sdk_official(&self) -> crate::Result<String> {
        if crate::get_device_api_level()? < 25 {
            return Err(crate::Error::ApiLevelTooLow);
        }

        let content_resolver = self.get_content_resolver()?;
        let setting_global = self.get_setting_global()?;
        let mut jni_env = global_jvm().get_env()?;

        let device_name = jni_env
            .get_static_field(&setting_global, "DEVICE_NAME", "Ljava/lang/String;")?
            .l()?;

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

    fn get_bluetooth_adapter_old(&self) -> crate::Result<JObject> {
        let bluetooth_adapter: JObject = global_jvm()
            .get_env()?
            .call_static_method(
                "android/bluetooth/BluetoothAdapter",
                "getDefaultAdapter",
                "()Landroid/bluetooth/BluetoothAdapter;",
                &[],
            )
            .expect("Failed to call getDefaultAdapter method")
            .l()?;

        if bluetooth_adapter.is_null() {
            return Err(crate::Error::BluetoothAdapterNull);
        }

        Ok(bluetooth_adapter)
    }

    fn get_bluetooth_adapter_new(&self) -> crate::Result<JObject> {
        let bluetooth_manager: JObject = global_jvm()
            .get_env()?
            .call_method(
                global_ctx(),
                "getSystemService",
                "(Ljava/lang/String;)Ljava/lang/Object;",
                &[JValue::Object(&self.get_str("bluetooth")?.into())],
            )
            .expect("Failed to call getSystemService method")
            .l()?;

        let bluetooth_adapter: JObject = global_jvm()
            .get_env()?
            .call_method(
                bluetooth_manager,
                "getAdapter",
                "()Landroid/bluetooth/BluetoothAdapter;",
                &[],
            )
            .expect("Failed to call getAdapter method")
            .l()?;

        if bluetooth_adapter.is_null() {
            return Err(crate::Error::BluetoothAdapterNull);
        }

        Ok(bluetooth_adapter)
    }

    /// lowest API level 1
    /// https://developer.android.com/reference/android/content/pm/PackageManager#checkPermission(java.lang.String,%20java.lang.String)
    pub fn check_permission_old<S: Into<String>>(&self, permission: S) -> crate::Result<()> {
        let mut jni_env = global_jvm().get_env()?;

        let permission_str = self.get_str(permission.into())?;
        let package_str = jni_env
            .call_method(global_ctx(), "getPackageName", "()Ljava/lang/String;", &[])
            .expect("Failed to call getPackageName method")
            .l()?;

        let package_manager = jni_env
            .call_method(
                global_ctx(),
                "getPackageManager",
                "()Landroid/content/pm/PackageManager;",
                &[],
            )
            .expect("Failed to call getPackageManager method")
            .l()?;

        let permission = jni_env
            .call_method(
                package_manager,
                "checkPermission",
                "(Ljava/lang/String;Ljava/lang/String;)I",
                &[
                    JValue::Object(&permission_str.into()),
                    JValue::Object(&package_str.into()),
                ],
            )
            // .expect("Failed to call checkSelfPermission method")
            .map_err(|e| {
                println!("checkSelfPermission error: {:?}", e);
                e
            })
            .expect("Failed to call checkSelfPermission method")
            .i()?;

        if permission != 0 {
            return Err(crate::Error::PermissionDenied);
        }

        Ok(())
    }

    /// lowest API level 23 but permission start on 23 so all return ok
    /// https://developer.android.com/reference/android/content/Context#checkSelfPermission(java.lang.String)
    pub fn check_permission_new<S: Into<String>>(&self, permission: S) -> crate::Result<()> {
        if crate::get_device_api_level()? < 23 {
            return Ok(());
        }

        let permission: String = permission.into();

        let mut jni_env = global_jvm().get_env()?;

        let permission_str = self.get_str(permission)?;

        let permission = jni_env
            .call_method(
                global_ctx(),
                "checkSelfPermission",
                "(Ljava/lang/String;)I",
                &[JValue::Object(&permission_str.into())],
            )
            .expect("Failed to call checkSelfPermission method")
            .i()?;

        if permission != 0 {
            return Err(crate::Error::PermissionDenied);
        }

        Ok(())
    }

    /// Update is reboot required
    /// * lowest API level 5
    /// * highest API level 30
    /// * Require BLUETOOTH permission
    /// * Require bluetooth feature
    /// * Android Emulator don't has bluetooth feature
    /// * BluetoothAdapter myDevice = BluetoothAdapter.getDefaultAdapter();
    /// * String deviceName = myDevice.getName();
    /// https://developer.android.com/reference/android/bluetooth/BluetoothAdapter#getDefaultAdapter()
    pub fn get_bluetooth_name_old(&self) -> crate::Result<String> {
        if crate::get_device_api_level()? < 5 {
            return Err(crate::Error::ApiLevelTooLow);
        }

        let permission = if (18..=29).contains(&crate::get_device_api_level()?) {
            "android.permission.BLUETOOTH"
        } else {
            "android.permission.BLUETOOTH_CONNECT"
        };
        if self.check_permission_old(permission).is_err() {
            return Err(crate::Error::PermissionDenied);
        }

        if self.check_permission_new(permission).is_err() {
            return Err(crate::Error::PermissionDenied);
        }

        let bluetooth_adapter = self.get_bluetooth_adapter_old()?;
        let mut jni_env = global_jvm().get_env()?;

        let device_name = jni_env
            .call_method(bluetooth_adapter, "getName", "()Ljava/lang/String;", &[])
            .expect("Failed to call getName method")
            .l()?
            .into();

        let device_name: String = jni_env.get_string(&device_name)?.into();

        Ok(device_name)
    }

    /// Update is reboot required
    /// * lowest API level 18
    /// * Require BLUETOOTH permission < API level 30
    /// * Require BLUETOOTH_CONNECT permission <
    /// * Require bluetooth feature
    /// * Android Emulator don't has bluetooth feature
    /// https://developer.android.com/reference/android/bluetooth/BluetoothManager#getAdapter()
    pub fn get_bluetooth_name_new(&self) -> crate::Result<String> {
        if crate::get_device_api_level()? < 18 {
            return Err(crate::Error::ApiLevelTooLow);
        }

        let bluetooth_adapter = self.get_bluetooth_adapter_new()?;
        let mut jni_env = global_jvm().get_env()?;

        let permission = if (18..=29).contains(&crate::get_device_api_level()?) {
            "android.permission.BLUETOOTH"
        } else {
            "android.permission.BLUETOOTH_CONNECT"
        };
        if self.check_permission_old(permission).is_err() {
            return Err(crate::Error::PermissionDenied);
        }

        if self.check_permission_new(permission).is_err() {
            return Err(crate::Error::PermissionDenied);
        }

        let device_name = jni_env
            .call_method(bluetooth_adapter, "getName", "()Ljava/lang/String;", &[])
            .expect("Failed to call getName method")
            .l()?
            .into();
        let device_name: String = jni_env.get_string(&device_name)?.into();

        Ok(device_name)
    }

    pub fn get_class_name<'other_local, O>(&self, obj: O) -> crate::Result<String>
    where
        O: AsRef<JObject<'other_local>>,
    {
        let mut jni_env = global_jvm().get_env()?;

        let class = jni_env.get_object_class(obj)?;

        let class_name_object = jni_env
            .call_method(&class, "getName", "()Ljava/lang/String;", &[])?
            .l()?
            .into();

        let class_name: String = jni_env.get_string(&class_name_object)?.into();

        Ok(class_name)
    }
}

// https://stackoverflow.com/questions/16704597/how-do-you-get-the-user-defined-device-name-in-android
// https://medium.com/@pribble88/how-to-get-an-android-device-nickname-4b4700b3068c
