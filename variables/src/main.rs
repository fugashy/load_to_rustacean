use std::fmt;

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
    // 個々の値を取り出す
    let (x, y, z) = tup;
    help_printing("x", x);
    help_printing("y", y);
    help_printing("z", z);
    let x = tup.0;
    help_printing("x", x);
    let y = tup.1;
    help_printing("y", y);
    let z = tup.2;
    help_printing("z", z);
}

// 固定長の配列はスタックに領域が確保される
fn list_type() {
    let a = [1, 2, 3, 4, 5];
    // プリントするにはstd::fmt::Displayトレイトが必要
    //  help_printing("a", a);
    let first = a[0];
    help_printing("first", first);
    let second = a[1];
    help_printing("second", second);
    // 無効なインデックスを使うとエラーで終了する
    //  let index = 5;
    //  let invalid_one = a[index];
    //  help_printing("invalid_one", invalid_one);
    // このときはコンパイル時点で教えてくれる
    //  let invalid_one = a[5];
    //  help_printing("invalid_one", invalid_one);
}

fn function_indcludes_statement_and_expression() {
    // yの中のxが評価されるので，このxは使われない
    // let x = 5;
    let y = {
        let x = 3;
        x + 1
    };
    help_printing("y", y);
}

fn function_returns_value() -> u32 {
    // 値として評価される例
    // セミコロンは不要
    // 経験的に，セミコロンが関数の最後についてなくても問題なかったりするので，
    // この辺の扱いがc/c++とは違うと思われる
    5 + 4
}

fn function_returns_value_as_return_word() -> u32 {
    // こちらは文として評価される例
    return 5 + 4;
}

struct MyTuple(i32, f64, u8);

// 独自の構造体のためのDisplayトレイトを実装する
impl std::fmt::Display for MyTuple {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.0, self.1, self.2)
    }
}

fn mytype() {
    let tup = MyTuple(500, 6.4, 1);
    help_printing("tup", tup);
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
    list_type();
    function_indcludes_statement_and_expression();
    help_printing("function_returns_value", function_returns_value());
    help_printing(
        "function_returns_value_as_return_word",
        function_returns_value_as_return_word(),
    );
    mytype();
}
