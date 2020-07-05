// まとめ
//   - あらゆるところでパターンは使用されている
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
    // 5はかならずしもxに束縛されている値とは限らない
    let x = 5;
    if let 5 = x {
        println!("5 matches x");
    }
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
