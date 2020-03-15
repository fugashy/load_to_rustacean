// std::fmt::Displayが定義されてないのでDebugで表示しながらすすめる
#[derive(Debug)]
enum IpAddrKind {
    // 異なる型の定義ができる!
    // 値・文字・構造体なんでもあり
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

// 使っていない属性をコンパイラに無視してもらう
// この記法は直後の定義についてのみ効果を発揮する
#[allow(dead_code)]
enum Message {
    // データなし
    Quit,
    // 匿名構造体
    Move { x: i32, y: i32 },
    // String
    Write(String),
    // 3つの値をもつ
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        return;
    }
}

fn use_message() {
    let m = Message::Write(String::from("hello"));
    m.call();
}

fn main() {
    simple_usage();
    use_message();
}
