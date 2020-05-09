use std::env;
use std::fs::File;
// 入出力処理に有用なトレイトを含んでいる
use std::io::prelude::*;
use std::process;

extern crate minigrep;
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    // unwrap_or_elseはErrの時にユーザ定義クロージャをコールしてくれる
    let c = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for: {}", c.query);
    println!("In file: {}", c.filename);

    let mut f = File::open(c.filename).expect("Failed to find");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}
