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

fn main() {
    test_largest_i32();
    test_largest_char();
    test_generic_largest();
}
