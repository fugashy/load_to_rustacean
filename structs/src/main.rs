struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn instanciate() {
    // メンバすべてが可変になる
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // c++同様ドット記法でメンバにアクセス
    user1.email = String::from("anotheremail@example.com");
    user1.active = false;
    user1.sign_in_count = 0;

    // _を変数名の頭につけるとunused variableを抑えることができる
    let _user2 = User {
        username: user1.username, // 他インスタンスの値を使用
        email: user1.email,
        active: false,
        sign_in_count: 2,
    };

    // フィールド初期化省略記法
    let username = String::from("hoge");
    let email = String::from("fuga");
    let _user3 = User {
        username,
        email,
        active: true,
        sign_in_count: 3,
    };
    println!("username of user3 is :{}", _user3.username);
    println!("email of user3 is :{}", _user3.email);

    // 構造体更新記法
    let _user4 = User {
        username: String::from("helloworld"),
        email: String::from("helloworld@example.com"),
        // あとは_user3と同じ
        .._user3
    };
}

// タプル構造体
// 名前のないフィールドを持つ
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn instanciate_tuple_structs() {
    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);
}

extern crate structs;
use structs::Rectangle;

fn instanciate_rectangle() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // Rustは複数のフィールドを持つユーザー定義型について推測しない
    println!("rect1 is {:#?}", rect1);
    println!("area of rect1 is {}", rect1.area());

    let rect2 = Rectangle {
        width: 20,
        height: 40,
    };
    println!("Can rect1 hold rect2? :{}", rect1.can_hold(&rect2));

    let rect3 = Rectangle::square(30);
    println!("rect3 is {:#?}", rect3);
}

struct Point2D {
    x: f64,
    y: f64,
}

impl Point2D {
    fn distance(&self, other: &Point2D) -> f64 {
        let x_squared = f64::powi(other.x - self.x, 2);
        let y_squared = f64::powi(other.y - self.y, 2);
        f64::sqrt(x_squared + y_squared)
    }
}

fn calcurate_distance_between_points() {
    let p1 = Point2D { x: 0.0, y: 0.0 };
    let p2 = Point2D { x: 5.0, y: 6.5 };

    println!("distance between p1 and p2 is :{}", p1.distance(&p2));
    // 等価
    println!("distance between p1 and p2 is :{}", (&p1).distance(&p2));
}

fn main() {
    instanciate();
    instanciate_tuple_structs();
    instanciate_rectangle();
    calcurate_distance_between_points();
}
