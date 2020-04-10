// コンパイラは標準でlib.rsのみを検索する
// 以下のモジュールを定義しておけばそのモジュール名に対応するファイルを作って行けば良くなる
// そういうふうにできている
pub mod client;
pub mod network;

#[cfg(test)]
mod tests {
    use super::client;
    #[test]
    fn it_works() {
        client::connect();
    }
}
