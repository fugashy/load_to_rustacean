fn if_statement() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

fn use_if_statement_in_let_statement() {
    let condition = true;
    let number = if condition {
        // 値として評価されたいのでセミコロンなし
        5
    } else {
        6
    };

    println!("the value of number is: {}", number);
}

fn loop_statement() {
    let mut count = 0;
    loop {
        count += 1;
        if count == 10 {
            break;
        }
        println!("again");
    }
    println!("out of loop");
}

fn while_statement() {
    let mut number = 3;

    while number != 0 {
        println!("{}", number);
        number -= 1;
    }
    println!("out of while");
}

fn scan_list_by_while_statement() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is {}:", a[index]);
        index += 1;
    }
}

fn for_statement() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is {}:", element);
    }
}

fn scan_range_type() {
    for number in 1..4 {
        println!("the number is {}:", number);
    }
}

fn main() {
    if_statement();
    use_if_statement_in_let_statement();
    loop_statement();
    while_statement();
    scan_list_by_while_statement();
    for_statement();
    scan_range_type();
}
