use std::ffi::{CStr, OsStr};
use std::os::windows::ffi::{OsStrExt, OsStringExt};
use std::{
    ffi::OsString,
    fmt::{Debug, Formatter},
};
use winapi::shared::minwindef::DWORD;
use winapi::shared::minwindef::MAKEWORD;
use winapi::um::iptypes::MAX_HOSTNAME_LEN;
use winapi::um::l2cmn::L2_PROFILE_MAX_NAME_LENGTH;
use winapi::um::sysinfoapi::GetComputerNameExW;
use winapi::um::sysinfoapi::{ComputerNamePhysicalDnsHostname, SetComputerNameExW};
use winapi::um::winnt;
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

    /// computer name change only on windows reboot
    /// This is not show on settings
    /// This is show on %COMPUTERNAME%
    /// get NetBIOS name
    /// This is just like get_a
    pub fn get_w(&self) -> crate::Result<String> {
        unsafe {
            // バッファサイズを定義
            let mut buffer: [winnt::WCHAR; L2_PROFILE_MAX_NAME_LENGTH + 1] =
                [0; L2_PROFILE_MAX_NAME_LENGTH + 1];
            let mut size: DWORD = buffer.len() as DWORD;

            // GetComputerNameWを呼び出してコンピュータ名を取得
            if winapi::um::winbase::GetComputerNameW(buffer.as_mut_ptr(), &mut size) != 0 {
                // Null終端のWCHARバッファをOsStringに変換
                let os_string = OsString::from_wide(&buffer[..(size as usize)]);

                // OsStringをStringに変換
                let computer_name = os_string.to_string_lossy();

                Ok(computer_name.into())
            } else {
                Err(std::io::Error::last_os_error().into())
            }
        }
    }

    /// computer name change only on windows reboot
    /// This is not show on settings
    /// This is show on %COMPUTERNAME%
    /// get NetBIOS name
    /// This is just like get_w
    pub fn get_a(&self) -> crate::Result<String> {
        unsafe {
            // バッファサイズを定義
            let mut buffer: [winnt::CHAR; L2_PROFILE_MAX_NAME_LENGTH + 1] =
                [0; L2_PROFILE_MAX_NAME_LENGTH + 1];
            let mut size: DWORD = buffer.len() as DWORD;

            // GetComputerNameAを呼び出してコンピュータ名を取得
            if winapi::um::winbase::GetComputerNameA(buffer.as_mut_ptr(), &mut size) != 0 {
                // Null終端のCHARバッファをOsStringに変換
                let computer_name = String::from_utf8_lossy(std::slice::from_raw_parts(
                    buffer.as_ptr() as *const u8,
                    size as usize,
                ));
                Ok(computer_name.into())
            } else {
                Err(std::io::Error::last_os_error().into())
            }
        }
    }

    /// computer name change only on windows reboot
    /// This is show on settings
    /// This is show on %COMPUTERNAME%
    /// DNS Host Name
    pub fn get_hostname(&self) -> crate::Result<String> {
        unsafe {
            let mut size: DWORD = 0;

            // DNS では長い名前が許可されるため、
            // サイズを取得する。
            // 必ず失敗する
            let result = GetComputerNameExW(
                ComputerNamePhysicalDnsHostname,
                std::ptr::null_mut(),
                &mut size,
            );
            if result != 0 {
                return Err(crate::Error::RuntimeError("unreachable".into()));
            }

            // バッファサイズを定義
            let mut buffer = Vec::with_capacity(size as usize);
            let _remaining = buffer.spare_capacity_mut();
            buffer.set_len(size as usize);

            // GetComputerNameExWを呼び出してコンピュータ名を取得
            if GetComputerNameExW(
                ComputerNamePhysicalDnsHostname,
                buffer.as_mut_ptr(),
                &mut size,
            ) != 0
            {
                // Null終端のWCHARバッファをOsStringに変換
                let os_string = OsString::from_wide(&buffer[..(size as usize)]);

                // OsStringをStringに変換
                let computer_name = os_string.to_string_lossy();

                Ok(computer_name.into())
            } else {
                Err(std::io::Error::last_os_error().into())
            }
        }
    }

    /// computer name change only on windows reboot
    /// This is show on settings pc name
    /// This is show on %COMPUTERNAME%
    /// but not official fn
    pub fn get_hostname_non_official(&self) -> crate::Result<String> {
        unsafe {
            // 初期化
            let mut wsadata: winapi::um::winsock2::WSADATA = std::mem::zeroed();
            if winapi::um::winsock2::WSAStartup(MAKEWORD(2, 2), &mut wsadata) != 0 {
                return Err(std::io::Error::last_os_error().into());
            }

            // ホスト名取得
            let mut buffer: [i8; MAX_HOSTNAME_LEN] = std::mem::zeroed();
            if winapi::um::winsock2::gethostname(buffer.as_mut_ptr(), buffer.len() as i32) != 0 {
                winapi::um::winsock2::WSACleanup();
                return Err(std::io::Error::last_os_error().into());
            }

            // クリーンアップ
            winapi::um::winsock2::WSACleanup();

            // i8からStringに変換
            let c_str = CStr::from_ptr(buffer.as_ptr());
            let hostname = c_str.to_string_lossy().into_owned();
            Ok(hostname)
        }
    }

    /// computer name change only on windows reboot
    pub fn get(&self) -> crate::Result<String> {
        self.get_hostname()
    }

    /// computer name change only on windows reboot
    /// This is not show on settings
    /// This is show on %COMPUTERNAME%
    /// set NetBIOS name
    /// This is just like set_a
    pub fn set_w<S: Into<String>>(&self, nickname: S) -> crate::Result<()> {
        let nickname: String = nickname.into();
        unsafe {
            let mut buffer = OsStr::new(&nickname).encode_wide().collect::<Vec<_>>();
            buffer.push(0);

            // GetComputerNameWを呼び出してコンピュータ名を取得
            if winapi::um::sysinfoapi::SetComputerNameW(buffer.as_ptr()) != 0 {
                Ok(())
            } else {
                Err(std::io::Error::last_os_error().into())
            }
        }
    }

    /// computer name change only on windows reboot
    /// This is not show on settings
    /// This is show on %COMPUTERNAME%
    /// set NetBIOS name
    /// This is just like set_w
    pub fn set_a<S: Into<String>>(&self, nickname: S) -> crate::Result<()> {
        let nickname: String = nickname.into();
        unsafe {
            // Convert OsStr to a String and obtain a slice of UTF-16 code units
            let cow = OsStr::new(&nickname).to_string_lossy();
            let mut buffer = cow.encode_utf16().map(|u| u as i8).collect::<Vec<i8>>();
            buffer.push(0);

            // SetComputerNameAを呼び出してコンピュータ名を設定
            if winapi::um::sysinfoapi::SetComputerNameA(buffer.as_ptr()) != 0 {
                Ok(())
            } else {
                Err(std::io::Error::last_os_error().into())
            }
        }
    }

    /// computer name change only on windows reboot
    /// This is show on settings
    /// This is show on %COMPUTERNAME%
    /// DNS Host Name
    pub fn set_hostname<S: Into<String>>(&self, nickname: S) -> crate::Result<()> {
        let nickname: String = nickname.into();

        unsafe {
            let mut buffer = OsStr::new(&nickname).encode_wide().collect::<Vec<_>>();
            buffer.push(0);

            // SetComputerNameExWを呼び出してコンピュータ名を設定
            if SetComputerNameExW(ComputerNamePhysicalDnsHostname, buffer.as_ptr()) != 0 {
                Ok(())
            } else {
                Err(std::io::Error::last_os_error().into())
            }
        }
    }

    /// computer name change only on windows reboot
    /// change windows and wsl name
    pub fn set<S: Into<String>>(&self, nickname: S) -> crate::Result<()> {
        self.set_hostname(nickname)
    }
}
