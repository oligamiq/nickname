# nick-name
![rustc >= 1.68.0](https://img.shields.io/badge/rustc-%3E%3D1.68.0-brightgreen)
![MIT/Apache 2](https://img.shields.io/crates/l/nick-name)
![ci](https://github.com/nziq53/nickname/actions/workflows/ci.yml/badge.svg)
![run](https://github.com/nziq53/nickname/actions/workflows/run.yml/badge.svg)

## What?
Nickname is Cross-Platform Rust user defined device-name(nick-name) get set library.

## use
```bash
cargo add nick-name
```
```Rust
let nickname = nick_name::NickName::new().unwrap();
let device_name = nickname.get().unwrap();
println!("{device_name}");

#[cfg(not(any(target_os = "ios", target_os = "android")))]
{
  let set = nickname.set("oligami-pc");
  println!("{:?}", set);

  let device_name = nickname.get().unwrap();
  println!("{device_name}");
}
```
Look at examples

## Support platforms
- linux systems
(Linux, FreeBSD, OpenBSD, NetBSD, Solaris, Redox, and so on)
- Windows
- Android
- iPhone

### Verify
https://doc.rust-lang.org/nightly/rustc/platform-support.html

`O` success<br>
`X` failed<br>
`-` non verify
|Target |build |test  |run   |clippy|fmt   |miri  |doc   |
|:----  |:----:|:----:|:----:|:----:|:----:|:----:|:----:|
|Tier1  |O     |O     |O     |O     |O     |O linux(get)<br>O Windows<br>X others|-|
|Android|O|O|O cargo-apk<br>X cargo-dinghy|O|O|X|-|
|iPhone |O|O|O cargo-bundle<br>O cargo-dinghy|O|O|X|-|
|Tier2  |O     |-     |-     |O     |O     |X     |-     |

### cannot support platform yet
- wasi
https://github.com/WebAssembly/wasi-libc/issues/196
- wasm

### next support platform yet
- watchOS
- visionOS
- WearOS
- BlueOS

## function
### Android
- get

### iPhone
- get

### Windows, Linux, macOS
- get
- set

! Windows don't set %username% yet

! macOS is not stable because not using native api yet

## build
look README.md on example
https://github.com/nziq53/nickname/tree/main/examples/common

## next
### all
- add test
※ zero tests(now)

- Match structure functions

### macOS
- use native api
- be able to `miri`

### Android
- be able to `miri`
- `check permission func` by using ndk
- add `require permission func`

### iPhone
- be able to `miri`

### linux
- be able to `miri` on set func

### apply rust new features ≧MSVC(1.68.0)
#### 1.74
##### Add promoting tier2 target
- [ ] add loongarch64-unknown-none

##### Use no-fail-test
- [ ] cargo test --tests --no-fail-fast

##### Raise minimum supported Apple OS versions
- [ ] macOS: 10.12 Sierra
- [ ] iOS: 10

#### 1.72
##### Add promoting tier2 target
- [x] add loongarch64-unknown-linux-gnu

## extension function
### Android
- finish<br>
call finishAndRemoveTask()
- get_device_api_level<br>
get api_level<br>
call VERSION.SDK_INT
- check_permission_old<br>
check permission
- check_permission_new<br>
check permission

## github actions test on mobile
### android
- api-level 24: `error: "API level 25 or higher is required"`
- api-level 30: `Android SDK built for x86_64`
- api-level 33: `sdk_gphone_x86_64`

## Platform information
### Android
#### Switch functions depending on API level
lowest API level 5
min API level < 25
- Android Emulator don't has bluetooth feature and cannot get nickname
- Require BLUETOOTH permission
- Update is required reboot

25 <= min API level
- Update is not required reboot
- Not require permission

#### Restriction
This library use native api by Android JavaVM.<br>
So, cargo dinghy and ffi binding must failed.

If you ffi binding, you call init func.

#### JavaVM
If you call using JavaVM func first, have JavaVM attach.
To attach JavaVM is heavy.
You drop NickName and free JavaVM attach.

# License
Licensed under either of
 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
 * BSD-3-clause ([LICENSE-BSD-3-clause](LICENSE-BSD-3-clause) or https://opensource.org/license/BSD-3-clause)
at your option.
