// vector, string, hashmapの使い方・注意点など

fn instantiate_vector() {
    // 素直?な宣言
    let _v: Vec<i32> = Vec::new();
    // マクロがある
    // 型推論してくれる
    let _v_macro = vec![1, 2, 3];

    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    // std::fmt::Displayがないので{:?}使う
    println!("new vector {:?}", v);
}

fn read_vector() {
    let v = vec![1, 2, 3, 4, 5];

    // 添字記法
    // 不変参照として得る
    // 参照として得る場合は，非参照側も&を明記する必要がある
    let third: &i32 = &v[2];
    println!("got {} by using [] accessing", third);
    // getメソッド
    // これはもともと不変参照を返すようだ
    let third: Option<&i32> = v.get(2);
    println!("got {} by using get method", third.unwrap());

    // パニックになる
    // 不正アクセスを即パニックにしたいときに有用
    // let does_not_exist = &v[100];
    // こちらはエラー処理
    let does_not_exist = v.get(100);
    if does_not_exist == None {
        println!("Invalid index");
    } else {
        println!("got {} by using get method", does_not_exist.unwrap());
    }
}

#[allow(unused_mut)]
fn try_to_push_to_ownershiped_vector() {
    // 可変
    let mut v = vec![1, 2, 3, 4, 5];

    // 不変
    let first = &v[0];

    // 不変借用されているオブジェクトに対して可変な処理ができない
    // 不変で借用されている状態だと，ヒープに変化があるかもしれないため許可されない
    // v.push(6);

    println!("borrowed {}", first);
    println!("try_to_push_to_ownershiped_vector {:?}", v);
}

fn scan_vector() {
    let mut v = vec![100, 32, 57];
    // 可変参照
    // こうじゃないと要素の値を変更できない
    for i in &mut v {
        // 参照外し演算子*が必要
        *i += 50;
    }
    // 不変参照
    // こうじゃなくてもよいが，多分こうしたほうがコピーが発生しない
    for i in &v {
        println!("{}", i);
    }
}

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn keep_enum() {
    // vecは一つの型しか保持できないが，enumが複数の型を表現できるため，rustでは割と自由
    let rows = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("brue")),
        SpreadsheetCell::Float(10.12),
    ];

    for row in &rows {
        println!("{:?}", row);
    }
}

// 文字列について
// 本当は結構複雑なんですよ
// rustでは言語の核としては
//  - 1種類しか文字列型は存在しない(文字列スライスのstr型)
// String型は核ではなく，標準ライブラリで提供される
//  - これは可変・伸長可能で所有権で管理される型
//  - UTF-8エンコード

fn instantiate_string() {
    let _data = "initial contents";

    let _s = _data.to_string();

    let s = "initial contents".to_string();

    println!("{}", s);
}

fn hello() {
    let hello = String::from("hogehoge");
    println!("{}", hello);
    let hello = String::from("Dobrý den");
    println!("{}", hello);
    let hello = String::from("Hello");
    println!("{}", hello);
    let hello = String::from("שלֹום");
    println!("{}", hello);
    let hello = String::from("नमस्ते");
    println!("{}", hello);
    let hello = String::from("こんにちは");
    println!("{}", hello);
}

fn push_str() {
    // 文字列スライスを追加してStringを伸ばす
    let mut s = String::from("foo");
    s.push_str("bat");
    println!("s is {}", s);

    let mut s1 = String::from("foo");
    let s2 = "bar";

    // push_strは所有権を奪わない
    s1.push_str(s2);
    println!("s1 is {}", s1);
    // 表示できる
    println!("s2 is {}", s2);
    // std::fmt::Pointerが実装されていないとアドレスの表示ができない
    println!("address of s2 is: {:p}", s2);
}

fn conbine() {
    let s1 = String::from("hello, ");
    let s2 = String::from("world");
    // +演算子は&strを取る関数
    // add(self, s: &str) -> String { ... }
    // 文字列の参照を追加する意味
    // 第2引数にあたえているs2は&Stringだが，型強制をしてくれる（される）
    // 第1引数selfは所有権を奪う->s1はmoveされる
    let s3 = s1 + &s2;
    // s1はmoveされるので使えない
    // Copyトレイトが実装されていないため
    // println!("s1 is {}", s1);
    println!("s2 is {}", s2);
    println!("s3 is {}", s3);
}

fn conbine_complex_string() {
    // 素でやるばあい
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("{}", s);
    // アウト
    // println!("{}", s1);

    // マクロを使う場合
    let s4 = String::from("tic");
    let s5 = String::from("tac");
    let s6 = String::from("toe");
    // これも所有権を奪わない
    let s = format!("{}-{}-{}", s4, s5, s6);
    println!("{}", s);
}

fn access_element_of_string() {
    let _s1 = String::from("hello");
    // rustでは添字アクセスをサポートしていない
    // let h = s1[0];

    // なぜかを調べる
    // 直感通り4bytes
    // utf-8でエンコードすると1byteとなる
    let len = String::from("Hola").len();
    println!("lenght of Hola is: {}", len);
    // 12と見せかけて24bytes
    // 各Unicodeスカラー値は2byteの領域を取る
    // ということで，添字記法は必ずしも有効なUnicodeのスカラー値と対応するわけではない
    let len = String::from("Здравствуйте").len();
    println!("lenght of Hola is: {}", len);

    // スライシングはできる
    let a = "Здравствуйте";
    println!("{}", &a[0..2]);
    // println!("{}", &a[0..3]); // これはだめ(境界ではないと言われる)
    println!("{}", &a[0..4]);
}

fn scan_string() {
    for c in "Hola".chars() {
        println!("{}", c);
    }
    for c in "Hola".bytes() {
        println!("{}", c);
    }
}

fn main() {
    instantiate_vector();
    read_vector();
    try_to_push_to_ownershiped_vector();
    scan_vector();
    keep_enum();
    instantiate_string();
    hello();
    push_str();
    conbine();
    conbine_complex_string();
    access_element_of_string();
    scan_string();
}
