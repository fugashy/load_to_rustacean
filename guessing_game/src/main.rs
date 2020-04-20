// randクレートを外部依存として使用することをコンパイラに伝える
extern crate rand;

// ライブラリをスコープする
// Rngトレイトが乱数生成器が実装するメソッドを定義しているためスコープする
// トレイトについてはまた...
use rand::Rng;
use std::cmp::Ordering;
use std::fmt;
use std::io;

pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Result<Guess, String> {
        if value < 1 || 100 < value {
            return Err(String::from("Guess value must be between 1 and 100"));
        }
        Ok(Guess { value })
    }
    pub fn value(&self) -> u32 {
        self.value
    }
}
impl std::fmt::Display for Guess {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "value: {}", self.value)
    }
}

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
