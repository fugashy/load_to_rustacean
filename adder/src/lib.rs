// ベンチマークテストもサポートしている
//
// また，ドキュメントに現れたコード例もコンパイルされる
// すごいな

// 以前作ったStructのテストをしてみる
pub struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

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

    use super::Rectangle;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            length: 8,
            width: 7,
        };
        let smaller = Rectangle {
            length: 5,
            width: 1,
        };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            length: 8,
            width: 7,
        };
        let smaller = Rectangle {
            length: 5,
            width: 1,
        };
        assert!(!smaller.can_hold(&larger));
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
