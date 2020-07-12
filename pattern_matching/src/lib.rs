// まとめ
//   - あらゆるところでパターンは使用されている
//   - ちゃんと，あり得る可能性をカバーしよう
//
// python 3.10あたりで，pythonにも導入されるかも?
// (rustのsubprocessはpythonから影響を受けていたりなど，インスパイアされあっているような雰囲気)
//
//
// この章は辞書的な感じで，いろんなパターンとそのマッチング方法を列挙している感じ
// まずは6章の軽い復習+α
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

pub fn simple_matching() {
    let coins = vec![Coin::Penny, Coin::Nickel, Coin::Dime, Coin::Quarter];

    for coin in coins.iter() {
        println!(
            "matched coin is {}",
            match coin {
                Coin::Penny => {
                    println!("Lucky penny!");
                    1
                }
                Coin::Nickel => 5,
                Coin::Dime => 10,
                Coin::Quarter => 25,
            }
        );
    }
    // Lucky penny!
    // matched coin is 1
    // matched coin is 5
    // matched coin is 10
    // matched coin is 25
}

pub fn all_patterns_must_be_covered() {
    let a = 3;
    let b = match a {
        1 => 1,
        2 => 2,
        3 => 3,
        _ => 0, // _パターンでカバーしないとだめ
                // そうしないと下のコンパイルエラー
                // patterns `std::i32::MIN..=0i32` and `4i32..=std::i32::MAX` not covered
    };
    println!("matched value is: {}", b);
}

// この例では輝かないが，if-let記法も道具箱にあるんですよ
// matchを使って表現するには冗長な場合に使いましょう
pub fn if_let_notation() {
    let a = 3;
    if let 1 = a {
        println!("matched value is: 1");
    } else if let 2 = a {
        println!("matched value is: 2");
    } else if let 3 = a {
        println!("matched value is: 3");
    } else {
        println!("matched value is: others");
    }
}

pub fn all_expressions_must_return_the_same_typed_value() {
    let a = 3;

    println!(
        "matched value is: {}",
        match a {
            1 => "one",
            2 => "two",
            // 3 => 3,  // expected '&str', found integer
            // 異なる型を返すように書くことはできない
            // どの型を基準にするかは，最初のパターンで返す型に依る
            _ => "others",
        }
    );
    // matched value is: others
}

// ここから18章の内容を本格的に

// ifとif-letが入り乱れるパターン
// お気に入りの色と年齢に応じて，背景色を変えることを考えているプログラム
// ややこしいからあまり使いたくないと感じた
pub fn mixed_if_statements() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
    // Using purple as the background color
}

// whileにもパターンを
// なにかある限りpopする例
pub fn pop_as_long_as_vector_has_values() {
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("vector has {} on the top", top);
    }
    // vector has 3 on the top
    // vector has 2 on the top
    // vector has 1 on the top
}

// forでもパターンの考え方は使われている
// for x in yでは，xがパターンとなる
// ここでは，タプルとして分解する方法を示す
pub fn for_statement_can_use_patterns() {
    let v = vec!['a', 'b', 'c'];

    for (i, v) in v.iter().enumerate() {
        println!("{} is at index {}", v, i);
    }
    // a is at index 0
    // a is at index 1
    // a is at index 2
}

// 実はいつもつかうlet文もパターンの考え方を使っている
pub fn let_statement_uses_pattern() {
    // let PATTERN = EXPRESSION;
    // ここでマッチしたものを変数_xに束縛することを意味するパターン
    // なんでもいいよと言っている
    let _x = 5;

    // mismatched type
    // expected a tuple with 3 elements, found one with 2 elements
    // let (_x, _y) = (1, 2, 3);

    // こうすると_x, _yのみに束縛できる(マッチングできる)
    let (_x, _y, _) = (1, 2, 3);
}

// 難所：論駁可能性について（ろんばくかのうせいについて）
// '論駁'を英辞郎で検索すると'反論'へ誘導される
// 論駁可能，不可能の２つの形態が，パターンにはある
#[allow(irrefutable_let_patterns)]
pub fn various_refutables() {
    // 論駁が可能なもの
    // なんらかの可能性がある値について，合致しないことがあるパターン
    let some_option_value = Some(3);
    // 合致しないパターンを考慮した制御フローを書かないとコンパイルエラーになる
    // 下のようにNoneの可能性を考慮して処理する
    if let Some(x) = some_option_value {
        println!("some option value is: {}", x);
    }

    // 論駁が不可能なもの
    // あらゆるものにマッチする
    // ワーニングがでる
    // irrefutable if-let pattern
    if let _x = 5 {
        println!("x matches 5");
    }

    // これは論駁可能
    // ワーニングはでない
    // 5はかならずしもxに束縛されている値と等しいわけではない
    let x = 5;
    if let 5 = x {
        println!("5 matches x");
    }
}

// あたらしいyを導入するアームのあるmatch式
#[allow(unreachable_patterns)]
pub fn match_arm_begins_new_scope() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(5) => println!("Got 50"),
        // 新しい変数yがあらわれた
        // 前に宣言しているyとは別物
        // このマッチアームはSome(x)の値がなんであれマッチするので，この処理が実行される
        // 外側のyと比較したい場合はマッチガード条件式を使用する必要がある
        Some(y) => println!("Matched, y = {:?}", y),
        // マッチガード条件式
        // これはパターンではないため，新しい変数を導入しない
        Some(n) if n == y => println!("Matched, n = {:?}", n),
        _ => println!("Default case, x = {:?}", x),
    }
}

// 複数のパターンにマッチさせる
pub fn multiple_match() {
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

// ..=で値の範囲に合致させる
pub fn range_match() {
    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }
    // one through five

    let x = 'k';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
    // late ASCII letter
}

struct Point2D {
    x: i32,
    y: i32,
}

pub fn decomposition_and_matching_of_struct_may_be_tricky() {
    let p = Point2D { x: 0, y: 7 };

    // 変数x, yとしてpの要素を取り出す
    let Point2D { x, y } = p;
    println!("x of Point is: {}", x); // 0
    println!("y of Point is: {}", y); // 7

    // match式で要素に応じた場合分け
    match p {
        // x軸上にいるか
        Point2D { x, y: 0 } => println!("On the x axis at: {}", x),
        // y軸上にいるか
        Point2D { x: 0, y } => println!("On the y axis at: {}", y), // 7
        // それ以外か
        Point2D { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

pub fn destructure_enum() {
    let msgs = vec![
        Message::Quit,
        Message::Move { x: 3, y: 4 },
        Message::Write("Hello!".to_string()),
        Message::ChangeColor(Color::Rgb(0, 255, 128)),
        Message::ChangeColor(Color::Hsv(0, 0, 0)),
    ];

    for msg in msgs.iter() {
        match msg {
            Message::Quit => {
                println!("The Quit variant has no data to destructure.");
            }
            Message::Move { x, y } => {
                println!("Move in the x direction {} and in the y direction {}", x, y);
            }
            Message::Write(text) => {
                println!("Text message: {}", text);
            }
            Message::ChangeColor(Color::Rgb(r, g, b)) => {
                println!("Change the color to red {}, green {}, blue {}", r, g, b);
            }
            Message::ChangeColor(Color::Hsv(h, s, v)) => {
                println!("Change the color to h {}, s {}, v {}", h, s, v);
            }
        }
    }
}

pub fn destructure_nested_structures() {
    let ((feet, inches), Point2D { x, y }) = ((3, 10), Point2D { x: 3, y: -10 });

    let vs = [("feet", feet), ("inches", inches), ("p.x", x), ("p.y", y)];
    for v in vs.iter() {
        println!("{:?}: {}", v.0, v.1);
    }
}

pub fn wild_card_in_the_nest() {
    // Someの中身がなんであれ無視(Someであれば無視)
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);
    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing costomized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting_value: {:?}", setting_value);
}

// _x記法は所有権を奪う
pub fn difference_between_unused_value_and_wild_card() {
    let s = Some(String::from("Hello?"));

    if let Some(_) = s {
        println!("found a string");
    }

    println!("the string: {:?}", s);

    // Someの中身はDropトレイトあり．Moveされる
    if let Some(_s) = s {
        println!("found a string");
    }

    // 使用不可
    // println!("the string: {:?}", s);
}

#[allow(dead_code)]
struct Point3D<T: std::fmt::Display> {
    x: T,
    y: T,
    z: T,
}
// ある範囲の部分を無視する
#[allow(unreachable_patterns)]
pub fn ignore_a_range_of_structure() {
    let p = Point3D::<f64> {
        x: 0.3,
        y: 6.45,
        z: -23.0,
    };

    match p {
        // xのみ興味がある書き方
        // それでも，なんでもよいと言っているようなもの
        // カバー漏れはないためコンパイル可能
        Point3D { x, .. } => {
            println!("x is {}", x);
        }
        // ちなみに，同じような守備範囲に思えるものを書いてもよさげ
        // こちらを先にかくと，こちらが実行される
        Point3D { x, y, .. } => {
            println!("x, y is {}, {}", x, y);
        }
    }
}

pub fn ignore_a_range_of_tuple_and_list() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        // ..は残りを省略するための書き方
        // 下のような，どこまでが残りなのかわからないような書き方はできない
        //  (.., second, ..) => {
        //      println!("second is ...: {}", second);
        //  }
        // こうしよう
        (_, second, ..) => {
            println!("second is ...: {}", second);
        }
    }
}

enum Msg {
    Hello { id: i32 },
}
// @バインディング
// 値を保持する変数の生成と同時にパターンに一致するか調べる
#[allow(unreachable_patterns)]
pub fn at_binding() {
    let msg = Msg::Hello { id: 5 };

    match msg {
        // 値を制限し，その範囲の値が含まれていることが保証される変数を生成
        Msg::Hello { id: id_val @ 3..=7 } => {
            println!("Found an id in range: {}", id_val);
        }
        // 変数の導入なし
        Msg::Hello { id: 5 } => {
            println!("Found an id: 5");
        }
        Msg::Hello { id } => {
            println!("Found some other id: {}", id);
        }
    }
}
