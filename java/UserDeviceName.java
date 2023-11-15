package com.oligami.rust_user_device_name_impl.android.impl;

import android.provider.Settings;
import android.content.Context;

@SuppressWarnings("unused") // Native code uses this class.
class UserDeviceName {
    private Context context;

    public UserDeviceName(Context context) {
        this.context = context;
    }

    public String getUserDeviceName() {
        String deviceName = Settings.Secure.getString(context.getContentResolver(), "bluetooth_name");
        return deviceName;
    }
}

// https://stackoverflow.com/questions/16704597/how-do-you-get-the-user-defined-device-name-in-android
// https://medium.com/@pribble88/how-to-get-an-android-device-nickname-4b4700b3068c

// Rustでコードを書くときの見本
