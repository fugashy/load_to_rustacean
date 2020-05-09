use std::env;
use std::process;

extern crate minigrep;
use minigrep::run;
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    // unwrap_or_elseはErrの時にユーザ定義クロージャをコールしてくれる
    let c = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // runがErrを返したかどうか
    if let Err(e) = run(c) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
