// ライブラリをスコープする
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    // mut予約後で変更可能な変数を宣言する
    // 空のStringオブジェクトに束縛されている可変変数を作っている
    let mut guess = String::new();

    // .read_lineではResultを返す
    // それはハンドリングされなければならない
    // .expect("xxx")でOkかErrかを判定しないといけない
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // {}はpythonでもおなじみなプレースホルダー
    println!("You guessed: {}", guess)
}
