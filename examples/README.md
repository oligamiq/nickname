# example
In order to see logs of the sample apps execute in a console:
```
adb logcat RustStdoutStderr:D '*:S'
```

# test
```
<!-- x devices -->

<!-- x run --device adb:16ee50bc -->

cargo apk run --example get
```

# install
```
<!-- cargo install xbuild -->
cargo install cargo-apk
```
