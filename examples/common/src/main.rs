fn main() {
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
}
