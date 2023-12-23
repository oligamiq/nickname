fn main() {
    let nickname = nick_name::NickName::new().unwrap();
    let device_name = nickname.get().unwrap();
    println!("{device_name}");

    #[cfg(target_os = "macos")]
    {
        let classes = nick_name::preview_all_classes();

        let device_name = nickname.get_by_NSImageNameComputer().unwrap();
        println!("NSImageNameComputer: {device_name}");

        let device_name = nickname.get_by_gethostname().unwrap();
        println!("gethostname: {device_name}");

        let device_name = nickname.get_by_sysctlbyname().unwrap();
        println!("sysctlbyname: {device_name}");

        let device_name = nickname.get().unwrap();
        println!("{device_name}");
    }

    // #[cfg(not(any(target_os = "ios", target_os = "android")))]
    // {
    //     let set = nickname.set("oligami-pc");
    //     println!("{:?}", set);

    //     let device_name = nickname.get().unwrap();
    //     println!("{device_name}");
    // }
}
