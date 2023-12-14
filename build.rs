fn main() {
    #[cfg(target_os = "ios")]
    {
        // println!("cargo:rustc-link-lib=framework=Foundation");

        println!("cargo:rustc-link-lib=framework=UIKit");

        // println!("cargo:rustc-link-lib=framework=CoreGraphics");
        // println!("cargo:rustc-link-lib=framework=QuartzCore");
        // println!("cargo:rustc-link-lib=framework=Security");
    }
}
