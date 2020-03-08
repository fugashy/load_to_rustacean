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

fn main() {
    if_statement();
    use_if_statement_in_let_statement();
    loop_statement();
    while_statement();
}
