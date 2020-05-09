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

    println!("With text:\n{}", contents);
    Ok(())
}
