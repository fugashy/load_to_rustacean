// std::fmt::Displayが定義されてないのでDebugで表示しながらすすめる
#[derive(Debug)]
enum IpAddrKind {
    V4(String),
    V6(String),
}

fn print(ip_type: IpAddrKind) {
    println!("type of ip is: {:?}", ip_type);
}

fn simple_usage() {
    let home = IpAddrKind::V4(String::from("127.0.0.1"));
    let loopback = IpAddrKind::V6(String::from("::1"));

    print(home);
    print(loopback);
}

fn main() {
    simple_usage();
}
