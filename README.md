# Supported platforms
- linux systems
(Linux, FreeBSD, OpenBSD, NetBSD, Solaris, Redox, and so on)
- Windows
- Android
- iPhone 

# cannot support platform yet
- wasi
https://github.com/WebAssembly/wasi-libc/issues/196
- wasm

# function
## Android
- get

## iPhone

## Windows, Linux, Mac
- get
- set

# Rust target
https://doc.rust-lang.org/nightly/rustc/platform-support.html

# build
look README.md on example
https://github.com/nziq53/nickname/tree/main/examples/common

# next
## Android
get nickname from bluetooth

## Macos
use native api

# extension function
## Android
- finish
call finishAndRemoveTask()
- get_device_api_level
get api_level
call VERSION.SDK_INT

# github actions test on mobile
## android
- api-level 24: `error: "API level 25 or higher is required"`
- api-level 30: `Android SDK built for x86_64`
- api-level 33: `sdk_gphone_x86_64`
