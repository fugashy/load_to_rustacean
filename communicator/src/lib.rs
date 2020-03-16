// コンパイラは標準でlib.rsのみを検索する
// 以下のモジュールを定義しておけばそのモジュール名に対応するファイルを作って行けば良くなる
// そういうふうにできている
mod client;
mod network;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
