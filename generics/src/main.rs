// listはi32のスライスを意味する
fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &number in list.iter() {
        if number > largest {
            largest = number;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &c in list.iter() {
        if c > largest {
            largest = c;
        }
    }
    largest
}

// 比較とコピーセマンティックを使用できる型のみに制限する
// コンパイラが教えてくれて，Tに求められることが明示されるのがいいな
fn largest<T: std::cmp::PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn test_largest_i32() {
    let number_list = vec![34, 50, 25, 100, 65];
    println!("The largest number is: {}", largest_i32(&number_list));
    let number_list = vec![102, 34, 6000, 89, 65];
    println!("The largest number is: {}", largest_i32(&number_list));
}

fn test_largest_char() {
    let char_list = vec!['y', 'm', 'a', 'q'];
    println!("The largest char is: {}", largest_char(&char_list));
}

fn test_generic_largest() {
    let number_list = vec![34, 50, 25, 100, 65];
    println!("The largest number is: {}", largest(&number_list));
    let char_list = vec!['y', 'm', 'a', 'q'];
    println!("The largest char is: {}", largest(&char_list));
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
    fn y(&self) -> &T {
        &self.y
    }
}

// 特定の型にのみ実装することもできる
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct Pair<T, U> {
    x: T,
    y: U,
}

impl<T, U> Pair<T, U> {
    // genericなクラスへの関数実装
    fn mixup<V, W>(self, other: Pair<V, W>) -> Pair<T, W> {
        Pair {
            x: self.x,
            y: other.y,
        }
    }
}

fn echo_point() {
    let p = Point { x: 5, y: 10 };
    println!("p.x is: {}", p.x());
    println!("p.y is: {}", p.y());
    // f32のみに実装された関数はコールできない
    // println!("distance_from_origin: {}", p.distance_from_origin());
    let p = Point { x: 5.0, y: 3.0 };
    println!("distance_from_origin: {}", p.distance_from_origin());
}

fn echo_pair() {
    let a = Pair { x: 4.0, y: 3 };
    let b = Pair { x: "Hello", y: 'c' };
    let c = a.mixup(b);
    println!("mixup: {}, {}", c.x, c.y);
}

fn main() {
    test_largest_i32();
    test_largest_char();
    test_generic_largest();
    echo_point();
    echo_pair();
}
