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

// Option<T>は標準ライブラリに含まれるenum
// 値が存在するかどうかをコード化し，null参照を未然に防ぐ狙いがある
// enum Option<T> {
//     Some(T),
//     None,
// }
// Some値があれば，値が存在することがわかり，その値はSomeに保持されている
fn option() {
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    println!("some_number is: {:?}", some_number);
    println!("some_string is: {:?}", some_string);
    println!("absent_number is: {:?}", absent_number);
}

//  fn invalid_operation_by_using_option() {
//      let x: i8 = 5;
//      let y: Option<i8> = Some(5);
//
//      // cannot add `std::option::Option<i8>` to `i8`
//      let sum = x + y;
//      println!("sum is: {}", sum);
//  }

#[derive(Debug)]
#[allow(dead_code)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quater(UsState),
}

#[derive(Debug)]
#[allow(dead_code)]
enum UsState {
    Alabama,
    Alaska,
    Calfornia,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        // 処理もかける
        Coin::Nickel => {
            println!("Nickel");
            5
        }
        Coin::Dime => 10,
        // 何か持っている場合は引数名を設定した上で使用可能!
        Coin::Quater(state) => {
            println!("State quater from: {:?}", state);
            25
        }
    }
}

fn inspect_coin() {
    let coin = Coin::Nickel;
    println!("value of coin is: {}", value_in_cents(coin));

    let coin = Coin::Quater(UsState::Alaska);
    println!("value of coin is: {}", value_in_cents(coin));
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn operation_of_option() {
    let five = Some(5);
    println!("five is {:?}", five);
    let six = plus_one(five);
    println!("six is {:?}", six);
    let none = plus_one(None);
    println!("none is {:?}", none);
}

fn main() {
    simple_usage();
    use_message();
    option();
    inspect_coin();
    operation_of_option();
}
