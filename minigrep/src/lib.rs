use std::env;
use std::error::Error;
use std::fs::File;
// 入出力処理に有用なトレイトを含んでいる
use std::io::prelude::*;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    // ライフタイム指定子をstaticにするのはなんでだろう
    // -> コンパイル時にサイズがわからないためとのこと
    // シグネチャとして，厳格にチェックされている（あとでせっていされていたとしても）
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        // プログラム名なのでスキップ
        // 消費してしまえ
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Failed to get query string"),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Failed to get filename string"),
        };

        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Searching for: {}", config.query);
    println!("In file: {}", config.filename);

    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");

    for line_contains_query in search(&config.query, &contents) {
        println!("{}", line_contains_query);
    }
    Ok(())
}

// Rust素人的には，素直なシグネチャで書いてしまう．
// 実際コンパイルすると，strはコンパイル時にサイズがわからないのでエラーになる
// ライフタイムについてジェネリックにならないと行けない
// この場合，入力されたコンテンツとアウトプットは共通のオブジェクトから借用されているはずなので，
// ライフタイムを一緒にする
fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // これはcontentsから生成されたStringが，このステートメントでDropするのでだめ
    // let c = contents.to_string().matches(query);

    // iteratorを使う前はおなじみのfor文で頑張っていた
    //  let mut result = Vec::new();
    //  // TODO(fugashy) str型が提供している関数を調べよう
    //  for line in contents.lines() {
    //      if line.contains(query) {
    //          result.push(line);
    //      }
    //  }
    //  result
    //
    // iteratorでfilterを使えば簡潔
    // こっちのほうが早いらしい
    // TODO(fugashy) ゼロ代償抽象化とやらを調べよう
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

#[cfg(test)]
mod unit_tests {
    use super::search;

    #[test]
    fn search_collectly() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(search(query, contents), vec!["safe, fast, productive."]);
    }
}
