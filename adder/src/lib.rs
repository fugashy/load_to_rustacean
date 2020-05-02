// ベンチマークテストもサポートしている
//
// また，ドキュメントに現れたコード例もコンパイルされる
// すごいな
#[cfg(test)]
mod tests {
    // テスト関数であることを示唆する
    #[test]
    fn exploration() {
        // アサーションマクロ
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn another() {
        panic!("Make this test fail");
    }
}
