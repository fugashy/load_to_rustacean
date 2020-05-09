// ベンチマークテストもサポートしている
//
// また，ドキュメントに現れたコード例もコンパイルされる
// すごいな
//
// 関数名を指定すると，その関数のみ実行される
// 指定した文字と合致するテストが実行される．これもすごい

// pubにしないと内部関数となる
// この場合，使われないとdead_codeワーニングがでる
pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("hello {}", name)
}

// 関数の出力はテストライブラリによってキャプチャされる(非表示)
// それは制御可能
pub fn prints_and_returns_10(a: i32) -> i32 {
    // cargo testのデフォルトは，この出力をキャプチャする
    // 表示したい場合は-- --nocaptureオプションを付加する
    println!("I got the value: {}", a);
    10
}

// 非公開
fn internal() -> String {
    String::from("I am internal test")
}

pub fn avoid_dead_code() {
    println!("{}", internal());
}

// 単体テストをするときはcfg(test)を使うことが慣例になっている
#[cfg(test)]
mod tests {
    // テスト関数であることを示唆する
    #[test]
    // 無視することもできる
    // #[ignore]
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

    use super::prints_and_returns_10;

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(0);
        assert_eq!(value, 10);
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(0);
        assert_eq!(value, 5);
    }

    use super::internal;

    // もちろん非公開関数のテストも単体テストの範疇
    #[test]
    fn validate_internal() {
        assert_eq!(internal(), "I am internal test");
    }
}
