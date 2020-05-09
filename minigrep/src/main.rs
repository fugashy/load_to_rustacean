use std::env;
use std::process;

extern crate minigrep;
use minigrep::run;
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    // unwrap_or_elseはErrの時にユーザ定義クロージャをコールしてくれる
    let c = Config::new(&args).unwrap_or_else(|err| {
        // 標準エラーストリームに出力
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // runがErrを返したかどうか
    if let Err(e) = run(c) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
