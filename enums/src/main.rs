// std::fmt::Displayが定義されてないのでDebugで表示しながらすすめる
#[derive(Debug)]
enum IpAddrKind {
    // 異なる型の定義ができる!
    V4(u8, u8, u8, u8),
    V6(String),
}

fn print(ip_type: IpAddrKind) {
    println!("type of ip is: {:?}", ip_type);
}

fn simple_usage() {
    let home = IpAddrKind::V4(127, 0, 0, 1);
    let loopback = IpAddrKind::V6(String::from("::1"));

    print(home);
    print(loopback);
}

fn main() {
    simple_usage();
}
