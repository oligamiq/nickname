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
        let hostname = self.get_hostname()?;
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

    pub fn set<S: Into<String>>(&self, nickname: S) -> crate::Result<()> {
        let nickname: String = nickname.into();
        self.set_hostname(nickname)?;
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
