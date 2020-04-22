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

fn main() {
    test_largest_i32();
    test_largest_char();
}
