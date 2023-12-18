fn main() {
    #[cfg(target_os = "ios")]
    {
        // println!("cargo:rustc-link-lib=framework=Foundation");

        println!("cargo:rustc-link-lib=framework=UIKit");

        // println!("cargo:rustc-link-lib=framework=CoreGraphics");
        // println!("cargo:rustc-link-lib=framework=QuartzCore");
        // println!("cargo:rustc-link-lib=framework=Security");
    }

    #[cfg(target_os = "macos")]
    {
        println!("cargo:rustc-link-lib=framework=Foundation");
        println!("cargo:rustc-link-lib=framework=AppKit");
        println!("cargo:rustc-link-lib=framework=CoreGraphics");
        println!("cargo:rustc-link-lib=framework=QuartzCore");
        println!("cargo:rustc-link-lib=framework=Security");
        // println!("cargo:rustc-link-lib=framework=UIKit");
        println!("cargo:rustc-link-lib=framework=WebKit");
        println!("cargo:rustc-link-lib=framework=CloudKit");
        println!("cargo:rustc-link-lib=framework=UserNotifications");
        println!("cargo:rustc-link-lib=framework=QuickLook");
    }
}
