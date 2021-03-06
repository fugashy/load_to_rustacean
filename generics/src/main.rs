// lib.rsで定義されたものを使うよ
extern crate generics;
// その中でも，Summaryも使うよ
use crate::generics::Summary;
// listはi32のスライスを意味する
fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &number in list.iter() {
        if number > largest {
            largest = number;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &c in list.iter() {
        if c > largest {
            largest = c;
        }
    }
    largest
}

// 比較とコピーセマンティックを使用できる型のみに制限する
// コンパイラが教えてくれて，Tに求められることが明示されるのがいいな
fn largest<T: std::cmp::PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn test_largest_i32() {
    let number_list = vec![34, 50, 25, 100, 65];
    println!("The largest number is: {}", largest_i32(&number_list));
    let number_list = vec![102, 34, 6000, 89, 65];
    println!("The largest number is: {}", largest_i32(&number_list));
}

fn test_largest_char() {
    let char_list = vec!['y', 'm', 'a', 'q'];
    println!("The largest char is: {}", largest_char(&char_list));
}

fn test_generic_largest() {
    let number_list = vec![34, 50, 25, 100, 65];
    println!("The largest number is: {}", largest(&number_list));
    let char_list = vec!['y', 'm', 'a', 'q'];
    println!("The largest char is: {}", largest(&char_list));
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
    fn y(&self) -> &T {
        &self.y
    }
}

// 特定の型にのみ実装することもできる
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct Pair<T, U> {
    x: T,
    y: U,
}

impl<T, U> Pair<T, U> {
    // genericなクラスへの関数実装
    fn mixup<V, W>(self, other: Pair<V, W>) -> Pair<T, W> {
        Pair {
            x: self.x,
            y: other.y,
        }
    }
}

fn echo_point() {
    let p = Point { x: 5, y: 10 };
    println!("p.x is: {}", p.x());
    println!("p.y is: {}", p.y());
    // f32のみに実装された関数はコールできない
    // println!("distance_from_origin: {}", p.distance_from_origin());
    let p = Point { x: 5.0, y: 3.0 };
    println!("distance_from_origin: {}", p.distance_from_origin());
}

fn echo_pair() {
    let a = Pair { x: 4.0, y: 3 };
    let b = Pair { x: "Hello", y: 'c' };
    let c = a.mixup(b);
    println!("mixup: {}, {}", c.x, c.y);
}

fn use_traits() {
    let tweet = generics::Tweet {
        username: String::from("fugashy"),
        content: String::from("Hello trait world"),
        reply: false,
        retweet: false,
    };
    println!("Summary of tweet: {}", tweet.summarize());
}

// 以降ではライフタイムについて触れる
// 目的は，ダングリング参照を回避すること

//  fn does_not_live_long_enough() {
//      let r;
//      {
//          let s = 5;
//          r = &s;
//          // ここでsはドロップされる
//      }
//      // rが参照するものはない
//      println!("r: {}", r);
//  }

// この関数は，実際にどの参照を返すのかが，実行前に判断できないので，
// 借用チェッカーが解析できない
// それぞれの生存期間が現時点で不明なため，ライフタイム指定子を要求する
// 'aで指定したライフタイム分だけ，両者は存在することをコンパイラに伝える
// そのような引数を与えることをユーザーに知らせる
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn get_longest() {
    let s1 = String::from("abdc");
    let s2 = "xyz";

    // ライフタイムの長さが一緒なので特に問題なし
    let r1 = longest(s1.as_str(), s2);
    println!("Longest string is: {}", r1);

    // 文字列スライスの生存期間を変えてみた
    // これは問題なし
    // 返した参照がs4だとしても，r2に返される場合はライフタイムがs3と等しくなるため大丈夫
    let s3 = String::from("ab");
    let r2;
    {
        // 文字列スライスはDropが実装されてない
        let s4 = "xyz";
        r2 = longest(s3.as_str(), &s4);
    }
    println!("Longest string is: {}", r2);

    // これはアウト
    // s6がDropすると，参照を返しても元が失われる
    //  let s5 = String::from("ab");
    //  let r3;
    //  {
    //      let s6 = String::from("xyz");
    //      r3 = longest(s5.as_str(), s6.as_str());
    //  }
    //  println!("Longest string is: {}", r3);
}

struct ImportantExcept<'a> {
    part: &'a str,
}

fn sample_of_lifetime_specified_struct() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcept {
        part: first_sentence,
    };
    println!("first_sentence is: {}", i.part);

    {
        let j = ImportantExcept {
            part: first_sentence,
        };
        println!("first_sentence is: {}", j.part);
    }
    let k;
    {
        k = ImportantExcept {
            part: first_sentence,
        };
    }
    println!("first_sentence is: {}", k.part);
}

// これまで参照を引数にする関数はたくさん作ってきたが，必ずしも，ライフタイムを指定する必要はなかった
// これは，借用チェッカーの実装によるところで，Rustの改善の歴史の中で進化してきたもの
// より省けるか，という点について改善がなされてきている
// そのルールについては気になったら都度調べよう
// どうせ忘れる
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: std::fmt::Display,
{
    println!("Announcement: {}", ann);

    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    test_largest_i32();
    test_largest_char();
    test_generic_largest();
    echo_point();
    echo_pair();
    use_traits();
    // does_not_live_long_enough();
    get_longest();
    sample_of_lifetime_specified_struct();
    longest_with_an_announcement("a", "b", "hogehoge");
}
