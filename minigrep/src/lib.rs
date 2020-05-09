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
