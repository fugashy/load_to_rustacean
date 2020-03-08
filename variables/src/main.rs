fn update() {
    // あとでいじりたいのでmutableにする
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}

fn shodowing() {
    let x = 5;
    // 6
    let x = x + 1;
    // 12
    let x = x * 2;
    println!("x is: {}", x);

    // 文字列型から数値型のシャドウイングも許される
    let spaces = "    ";
    let spaces = spaces.len();
    println!("length of spaces is :{}", spaces);
}

// 型を変化させるような更新は許されない
//  fn not_allowed_shudowing() {
//      let mut spaces = "    ";
//      spaces = spaces.len();
//      println!("length of spaces is :{}", spaces);
//  }

fn main() {
    update();
    shodowing();
}
