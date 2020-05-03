// ベンチマークテストもサポートしている
//
// また，ドキュメントに現れたコード例もコンパイルされる
// すごいな

// pubにしないと内部関数となる
// この場合，使われないとdead_codeワーニングがでる
pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("hello {}", name)
}

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

    use super::add_two;

    #[test]
    fn it_add_two() {
        assert_eq!(4, add_two(2));
    }

    use super::greeting;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was '{}'",
            result
        );
    }
}
