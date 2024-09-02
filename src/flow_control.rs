

fn max_conditional_expression(arg1: i32, arg2: i32) -> i32{
    if arg1 > arg2 {
        arg1
    } else {
        arg2
    }
}

fn max_conditional_statement(arg1: i32, arg2: i32) -> i32{
    // if is an expression so we can write
    let max = if arg1 > arg2 {
        arg1
    } else {
        arg2
    };
    max
}

fn nested_condition(arg1: i32, arg2: i32) -> bool{
    if arg1 % 2 == 0 {
        println!("arg1 is even {}", arg1);
        true
    } else {
        println!("arg1 is odd {}", arg1);
        if arg2 % 2 == 0 {
            println!("arg2 is even {}", arg2);
            return true;
        } else {
            println!("arg2 is odd {}", arg2);
            return false;
        }
    }
}

fn and_or() -> (bool, bool) {
    let a = false;
    let b = true;

    return (a && b, a || b);
}

fn match_enums(){
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }

    println!("Penny in cents: {}", value_in_cents(Coin::Penny));
    println!("Nickel in cents: {}", value_in_cents(Coin::Nickel));
    println!("Dime in cents: {}", value_in_cents(Coin::Dime));
    println!("Quarter in cents: {}", value_in_cents(Coin::Quarter));
}

fn conditionless_loop(){
    // loop {
    //     Endless loop
    // }

    let mut counter = 0;
    
    let result = loop {
        if counter < 10 {
            println!("counter is {}", counter);
            counter += 1;
        } else {
            break counter;
        }
    };

    println!("The result is: {}", result);
}

fn while_loop() {
    let mut count = 3;
    while count/2 < 5 {
        println!("while loop count is {}", count);
        count += 1;
    }
}

fn for_iterator(){
    let a = [1,2,3,4,5,6,7,8,9];
    let b = "hello";

    for element in a {
        println!("iterator element is: {}", element);
    }

    for char in b.chars() {
        println!("iterator element is: {}", char);
    }

    for range_number in 1..4 {
        println!("range iterator value is: {}", range_number)
    }
}

fn fizz_buzz_exercise(){
    println!("Start");
    for number in 1..100 {
        let mut str: String = "".to_owned();
        if number % 3 == 0 {
            str.push_str("Fizz");
        }
        if number % 5 == 0 {
            str.push_str("Buzz");
        }
        if str != "" {
            println!("{}", str);
        }
    }
}

pub fn flow_control(){
    let max1 = max_conditional_expression(5, 4);
    println!("max_conditional_expression value is {}", max1);
    
    let max2 = max_conditional_statement(5, 4);
    println!("max_conditional_statement value is {}", max2);
    
    let oddity = nested_condition(5, 4);
    println!("oddity is {}", oddity);

    let (a_and_b, a_or_b) = and_or();
    println!("a or b is true: {}", a_or_b);
    println!("a and b is true: {}", a_and_b);

    match_enums();

    conditionless_loop();

    while_loop();

    for_iterator();

    fizz_buzz_exercise();
}
