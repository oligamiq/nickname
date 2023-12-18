// https://github.com/svartalf/hostname/blob/master/src/nix.rs

use std::ffi::CStr;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::os::unix::ffi::OsStrExt;
// use std::os::unix::process::CommandExt;
use std::sync::Arc;
use std::sync::RwLock;

use libc::sysctlbyname;
use objc::{class, msg_send};

use crate::macos::util::id;

use super::preview_all_classes;
const _POSIX_HOST_NAME_MAX: libc::c_long = 255;

pub struct NickName(pub Arc<RwLock<id>>);

impl Debug for NickName {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        f.debug_struct("NickName").finish()
    }
}

// #[link(name = "AppKit", kind = "framework")]
// extern "system" {
//     static NSImageNameComputer: *const std::ffi::c_char;
// }

impl NickName {
    pub fn new() -> crate::Result<Self> {
        preview_all_classes();

        // let name = unsafe { CStr::from_ptr(NSImageNameComputer.into()) };

        // println!("global name: {}", name.to_str().unwrap());

        Ok(Self(Arc::new(RwLock::new(unsafe {
            // Create an instance of the UIDevice class
            let superclass = class!(NSObject);
            // デバッグ用にsuperclassの情報を出力する
            println!("superclass: {:?}", superclass);

            // let panel = class!(NSWindow);
            // println!("panel: {:?}", panel);

            objc::runtime::AnyClass::get("NSObject").unwrap();

            let mut cmd = std::process::Command::new("scutil --get ComputerName");
            let out = cmd.output().unwrap().stdout;
            let out = String::from_utf8(out).unwrap();
            println!("out: {}", out);

            msg_send![class!(NSObject), alloc]
        }))))
    }

    pub fn get(&self) -> crate::Result<String> {
        let hostname = self.get_hostname()?;

        let name = self.get_name()?;
        println!("name: {}", name);

        Ok(hostname)
    }

    pub fn get_hostname(&self) -> crate::Result<String> {
        // ホスト名を格納するバッファのサイズを指定
        // https://pubs.opengroup.org/onlinepubs/9699919799/functions/gethostname.html
        let limit = unsafe { libc::sysconf(libc::_SC_HOST_NAME_MAX) };
        let size = libc::c_long::max(limit, _POSIX_HOST_NAME_MAX) as usize;
        let mut hostname_buffer: Vec<u8> = vec![0; size + 1];

        // libcのgethostname関数を呼び出し、ホスト名を取得
        let result =
            unsafe { libc::gethostname(hostname_buffer.as_mut_ptr() as *mut libc::c_char, size) };

        if result != 0 {
            return Err(std::io::Error::last_os_error().into());
        }

        // ヌル終端されたC文字列をRustの文字列に変換
        let hostname_cstr =
            unsafe { CStr::from_ptr(hostname_buffer.as_ptr() as *const libc::c_char) };
        match hostname_cstr.to_str() {
            Ok(v) => Ok(v.into()),
            Err(e) => Err(crate::Error::Other(Box::new(e))),
        }
    }

    fn get_name(&self) -> crate::Result<String> {
        let mut mib: [libc::c_int; 2] = [0, 0];
        let mut len: libc::size_t = 0;

        mib[0] = libc::CTL_KERN;
        mib[1] = libc::KERN_HOSTNAME;

        let result = unsafe {
            sysctlbyname(
                "kern.hostname\0".as_ptr() as *const libc::c_char,
                std::ptr::null_mut(),
                &mut len,
                std::ptr::null_mut(),
                0,
            )
        };

        if result != 0 {
            return Err(std::io::Error::last_os_error().into());
        }

        let mut hostname_buffer: Vec<u8> = vec![0; len + 1];

        let result = unsafe {
            sysctlbyname(
                "kern.hostname\0".as_ptr() as *const libc::c_char,
                hostname_buffer.as_mut_ptr() as *mut libc::c_void,
                &mut len,
                std::ptr::null_mut(),
                0,
            )
        };

        if result != 0 {
            return Err(std::io::Error::last_os_error().into());
        }

        let hostname_cstr =
            unsafe { CStr::from_ptr(hostname_buffer.as_ptr() as *const libc::c_char) };
        let hostname = hostname_cstr.to_str().unwrap();

        Ok(hostname.into())
    }

    pub fn set<S: Into<String>>(&self, nickname: S) -> crate::Result<()> {
        let nickname: String = nickname.into();
        self.set_hostname(nickname)?;

        let name = self.get_name()?;
        println!("set name: {}", name);

        Ok(())
    }

    // https://github.com/svartalf/hostname/blob/master/src/nix.rs
    pub fn set_hostname<S: Into<String>>(&self, nickname: S) -> crate::Result<()> {
        let nickname: String = nickname.into();
        let nickname = std::ffi::OsStr::new(&nickname);

        if nickname.len() > libc::c_int::MAX as usize {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "nickname too long").into());
        }

        let size = nickname.len() as libc::c_int;

        let result =
            unsafe { libc::sethostname(nickname.as_bytes().as_ptr() as *const libc::c_char, size) };

        if result != 0 {
            Err(std::io::Error::last_os_error().into())
        } else {
            Ok(())
        }
    }
}
