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

fn main() {
    instantiate_vector();
    read_vector();
    try_to_push_to_ownershiped_vector();
    scan_vector();
    keep_enum();
}
