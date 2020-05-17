use std::env;
use std::process;

extern crate minigrep;
use minigrep::run;
use minigrep::Config;

fn main() {
    // unwrap_or_elseはErrの時にユーザ定義クロージャをコールしてくれる
    // env::argsから得られるイテレータの所有権を渡してしまう
    let c = Config::new(env::args()).unwrap_or_else(|err| {
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
