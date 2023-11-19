# Supported platforms
- POSIX-compliant systems
(Linux, macOS, Android, FreeBSD, OpenBSD, NetBSD, Solaris, Redox, and so on)
- Windows
- Android

# cannot support platform yet
- wasi
https://github.com/WebAssembly/wasi-libc/issues/196
- wasm

# function
## android
- get

## Windows, Linux, Mac
- get
- set

# rust target
https://doc.rust-lang.org/nightly/rustc/platform-support.html

# build
look README.md on example

# next
## android
get nickname from bluetooth

# extension function
## android
- finish
call finishAndRemoveTask()
- get_device_api_level
get api_level
call VERSION.SDK_INT

# github actions test on mobile
## android
api-level 24: `error: "API level 25 or higher is required"`
api-level 30: `Android SDK built for x86_64`
api-level 33: `sdk_gphone_x86_64`
