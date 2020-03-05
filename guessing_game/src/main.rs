// randクレートを外部依存として使用することをコンパイラに伝える
extern crate rand;

// ライブラリをスコープする
// Rngトレイトが乱数生成器が実装するメソッドを定義しているためスコープする
// トレイトについてはまた...
use rand::Rng;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}", secret_number);

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
    println!("You guessed: {}", guess);
}
