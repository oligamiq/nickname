// https://github.com/svartalf/hostname/blob/master/src/nix.rs

use std::ffi::CStr;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::os::unix::ffi::OsStrExt;
const _POSIX_HOST_NAME_MAX: libc::c_long = 255;

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

    pub fn get(&self) -> crate::Result<String> {
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

    // https://github.com/svartalf/hostname/blob/master/src/nix.rs
    pub fn set<S: Into<String>>(&self, nickname: S) -> crate::Result<()> {
        let nickname: String = nickname.into();
        let nickname = std::ffi::OsStr::new(&nickname);

        #[cfg(not(any(
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "ios",
            target_os = "macos",
            target_os = "solarish",
            target_os = "illumos",
        )))]
        #[allow(non_camel_case_types)]
        type nickname_len_t = libc::size_t;

        #[cfg(any(
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "ios",
            target_os = "macos",
            target_os = "solarish",
            target_os = "illumos",
        ))]
        #[allow(non_camel_case_types)]
        type nickname_len_t = libc::c_int;

        #[cfg(not(any(
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "ios",
            target_os = "macos",
            target_os = "solarish",
            target_os = "illumos",
        )))]
        #[allow(clippy::absurd_extreme_comparisons)]
        if nickname.len() > nickname_len_t::MAX {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "nickname too long").into());
        }

        #[cfg(any(
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "ios",
            target_os = "macos",
            target_os = "solarish",
            target_os = "illumos",
        ))]
        if nickname.len() > nickname_len_t::MAX as usize {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "nickname too long").into());
        }

        let size = nickname.len() as nickname_len_t;

        let result =
            unsafe { libc::sethostname(nickname.as_bytes().as_ptr() as *const libc::c_char, size) };

        if result != 0 {
            Err(std::io::Error::last_os_error().into())
        } else {
            Ok(())
        }
    }
}
