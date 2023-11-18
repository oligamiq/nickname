## build
cargo build --target wasm32-wasi
wasmer run

# not support
wasi-libc is not support hostname
https://github.com/WebAssembly/wasi-libc/issues/196

# init
rustup add target wasm32-wasi
