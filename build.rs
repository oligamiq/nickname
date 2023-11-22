use std::env;
use std::path::PathBuf;

fn sdk_path(target: &str) -> Result<String, std::io::Error> {
    use std::process::Command;
    let sdk = if vec![
        "x86_64-apple-ios",
        "i386-apple-ios",
        "aarch64-apple-ios-sim",
    ]
    .contains(&target)
    {
        "iphonesimulator"
    } else if target == "aarch64-apple-ios"
        || target == "armv7-apple-ios"
        || target == "armv7s-apple-ios"
    {
        "iphoneos"
    } else {
        unreachable!();
    };

    let output = Command::new("xcrun")
        .args(&["--sdk", sdk, "--show-sdk-path"])
        .output()?
        .stdout;
    let prefix_str = std::str::from_utf8(&output).expect("invalid output from `xcrun`");
    Ok(prefix_str.trim_end().to_string())
}

fn main() {
    #[cfg(target_os = "ios")]
    println!("cargo:rustc-link-lib=framework=UIKit");
}
