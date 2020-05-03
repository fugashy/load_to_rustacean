// Debugを継承することでstd::fmt::Debugが提供される
#[derive(Debug)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}
impl Rectangle {
    // Rectangleの文脈中にあることがコンパイラが理解できるので，selfでよい
    // 自分自身を不変参照で借用している
    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
// implブロックでは内部関数だけでなく，関連する関数も定義できる
// 複数のimplブロックを作ることもできる
// いつか役に立つ
impl Rectangle {
    pub fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Rectangle;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            height: 8,
            width: 7,
        };
        let smaller = Rectangle {
            height: 5,
            width: 1,
        };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            height: 8,
            width: 7,
        };
        let smaller = Rectangle {
            height: 5,
            width: 1,
        };
        assert!(!smaller.can_hold(&larger));
    }
}
