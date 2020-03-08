// Tに対しては，Displayトレイトが実装されているものに限る（トレイト境界）
fn help_printing<T: std::fmt::Display>(name: &str, value: T) {
    println!("The value of {0} is: {1}", name, value);
}

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

fn numerical_operations() {
    let sum = 5 + 10;
    help_printing("sum", sum);
    let difference = 99.5 - 4.3;
    help_printing("difference", difference);
    let product = 4 * 30;
    help_printing("product", product);
    let quotient = 56.7 / 33.2;
    help_printing("quotient", quotient);
    let remainder = 43 % 5;
    help_printing("remainder", remainder);
}

fn logistic_operations() {
    let t = true;
    help_printing("t", t);
    let f: bool = false;
    help_printing("f", f)
}

fn char_type() {
    let c = 'z';
    help_printing("c", c);
    let z = 'Ｚ';
    help_printing("z", z);
    let heart_eyed_cat = '😻';
    help_printing("heart_eyed_cat", heart_eyed_cat);
}

fn tuple_type() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    //  help_printing("tup", tup);
    // ここの値を取り出す
    let (x, y, z) = tup;
    help_printing("x", x);
    help_printing("y", y);
    help_printing("z", z);
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
    numerical_operations();
    logistic_operations();
    char_type();
    tuple_type();
}
