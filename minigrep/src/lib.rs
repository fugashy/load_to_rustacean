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
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments.");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

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

    let _r = search(&config.query, &contents);

    println!("With text:\n{}", contents);
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

    let mut result = Vec::new();
    // TODO(fugashy) str型が提供している関数を調べよう
    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    result
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
