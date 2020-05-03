// randクレートを外部依存として使用することをコンパイラに伝える
extern crate rand;

// ライブラリをスコープする
// Rngトレイトが乱数生成器が実装するメソッドを定義しているためスコープする
// トレイトについてはまた...
use rand::Rng;
use std::cmp::Ordering;
use std::io;

extern crate guessing_game;
use guessing_game::Guess;

fn main() {
    println!("Guess the number!");

    loop {
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

        // shadowingしてもとの値を覆い隠す
        // guess_strみたいなカッコ悪いことはしなくてもよい！
        // trim()は\nを削除してくれる
        // parse()はなんらかの数値にしてくれる
        // match式はエラー処理の一般的な方法
        let num: u32 = match guess.trim().parse() {
            Ok(num) => num,
            // _は包括値（どんなものでも構わないという意思表示
            Err(_) => {
                println!("input value");
                continue;
            }
        };

        // 意図しない数の場合は再入力を要求するようにする
        // the bookでは不正な値はpanicにしているが，このプログラムでは再入力のほうがいいと思った
        let guess: Guess = match Guess::new(num) {
            Ok(g) => g,
            Err(s) => {
                println!("{}", s);
                continue;
            }
        };

        // {}はpythonでもおなじみなプレースホルダー
        println!("You guessed: {}", guess.value());

        let secret_number = rand::thread_rng().gen_range(1, 101);
        match guess.value().cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
