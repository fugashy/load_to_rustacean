// 結合テスト
// 公開されているものしかさわれない
extern crate adder;

// cargo test integration_test
mod integration_test {
    #[test]
    fn it_will_fail() {
        assert_eq!(adder::add_two(3), 3);
    }

    #[test]
    fn it_will_pass() {
        assert_eq!(adder::add_two(3), 5);
    }
}
