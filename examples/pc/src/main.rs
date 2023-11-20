fn main() {
    let nickname = nick_name::NickName::new().unwrap();
    let device_name = nickname.get().unwrap();
    println!("{device_name}");

    let set = nickname.set("oligami-pc");
    println!("{:?}", set);();

    let device_name = nickname.get().unwrap();
    println!("{device_name}");
}
