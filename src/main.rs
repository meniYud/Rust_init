// use std::char;

fn vars() {
    {
        //types
        let mut small_int = 21648;
        println!("The value of small_int is {}, which is i32", small_int);
    
        let big_int: i64 = 42587621648;
        println!("The value of big_int is {}, which is i64", big_int);
    
        small_int = 5246;
        println!("small_int value is now {}. It is a mutable thanks to the 'mut' operator", small_int);
    }
    
    {
        //shadowing
    
        let x = 5;
        println!("The value of x is {}", x);
    
        {
            let x = 6;
            println!("The value of x inside this scope is {}", x);
        }
    
        let x = x + 2;
        println!("Back to the main scope, the value of x is now {}", x);
    }

    {
        //mutability meet types
        let a = 5;
        println!("Before mutation, a is a number with value {}", a);

        let a = "Holla";
        println!("After mutation, a is a string with value {}", a);

        let mut b = 5;
        println!("Since b is mutable variable - it cannot switch types");

        // b = "NO";
    }

    {
        //constants
        const MAX_POINTS:i32 = 100_000;
        println!("The value of MAX_POINTS must be initialized as part of declaration {}", MAX_POINTS);

        {
            const MAX_POINTS:i32 = 100_001;
            println!("The value of MAX_POINTS can be override in an inner scope {}", MAX_POINTS);
        }
    }


}

fn data_types() {
// scalar types
    {
        //  - Integers
        let small_number: u8 = 255;
        let big_number: u128 = 123456789012345678;

        let signed_small_number: i8 = -127;
        let signed_big_number: i128 = -123456789012345678;

        println!("small_number: {}", small_number);
        println!("big_number: {}", big_number);
        println!("signed_small_number: {}", signed_small_number);
        println!("signed_big_number: {}", signed_big_number);

        let decimal = 98_167;
        let hex = 0xff;
        let octal = 0o77;
        let binary = 0b1111_0000;
        let byte = b'A';
        
        println!("Decimal: {}", decimal);
        println!("hex: {}", hex);
        println!("octal: {}", octal);
        println!("binary: {}", binary);
        println!("byte: {}", byte);
    }
    
    {
        //  - Floating-point numbers
        let x = 2.0; // f64 by default. if you comment the arithmetic sections bellow - watch the default type changes
        let y: f32 = 3.0;

        println!("x= {}, y = {}", x, y);

        let sum = x + y;
        let difference = x - y;
        let product = x * y;
        let division = x / y;
        let reminder = x % y;

        println!("sum: {}", sum);
        println!("difference: {}", difference);
        println!("product: {}", product);
        println!("division: {}", division);
        println!("reminder: {}", reminder);
    }
    
    {
        //  - Booleans
        let t = true;
        let f = false;

        println!("t is {}, f is {}", t, f);

        let not_t = !t;

        println!("not_t = {}", not_t);
    }
    
    {
        //  - Characters
        let c1 = "z";
        let c2 = "X";
        let c3 = "😂";

        println!("c1 is {}, c2 is {}, c3 is {}", c1, c2 ,c3);

        //iterate over characters in a string
        let str = "Ciao, Hola, Bye";
        for char in str.chars() {
            println!("{}", char);
        }
    }

    // compound types
    {
        //  - Tuples
        let tup: (i32, f64, char) = (500, 6.4, 'x');

        let (x, y, z) = tup;
        println!("The value of x is {}", x);
        println!("The value of y is {}", y);
        println!("The value of z is {}", z);
        
        println!("The first value is {}", tup.0);
        println!("The second value is {}", tup.1);
        println!("The third value is {}", tup.2);
    }
    
    {
        //  - Arrays
        let arr = [1,2,3,4,5]; // fixed size!! all of same type!!
        let first = arr[0];
        let second = arr[1];

        println!("First is {}, Second is {}", first, second);

        //iteration
        for element in arr.iter() {
            println!("element: {}", element);
        }
    }

    // custom types

    {
        //  - Structs
        struct Person {
            name: String,
            age: u8,
        }

        let person = Person {
            name: String::from("John"),
            age: 25,
        };

        println!("person's name is: {}, and his age is: {}", person.name, person.age);

    }
    
    {
        //  - Enums
        enum TrafficLight {
            Red,
            Yellow,
            Green,
        }

        let go_light = TrafficLight::Green;
        
        match go_light {
            TrafficLight::Red => println!("Stop!"),
            TrafficLight::Yellow => println!("Slow down"),
            TrafficLight::Green => println!("Go!"),
        }

    }
}

fn another_function(num: i32, letter: char, str: &str){
    println!("The order of functions declarations is not important");
    println!("The argument of num (value of the parameter called num) is {}", num);
    println!("The argument of letter (value of the parameter called num) is {}", letter);
    println!("The argument of str (value of the parameter called num) is {}", str);
}

fn sum(num1: i32, num2: i32) -> i32 {
    num1 + num2
}
fn sum_and_sub(num1: i32, num2: i32) -> (i32, i32) {
    return (num1 + num2, num1 - num2);
    println!("This will not be printed");
}

fn statements_vs_expressions(){
    let x = 42; //statement (does not return value)
    let z = 9 + x; //The RHS is an expression (returns a value)

    // block scopes are expression
    let y = {
        let x = 3;
        
        x + 1
        // NOT x + 1;
    };
    println!("The returned value of the scope is {}", y);
    println!("Note: if you add semi;colons at the end of line 221, the block scope will become a statement rather than expression");

    //Functions are also expressions
    println!("The returned value is {}", sum(5,7));
    println!("Note: if you add semi;colons at the end of line 206, the block scope will become a statement rather than expression");

    let ret_val2 = sum_and_sub(5,7);
    println!("You can also return a tupple: ({}, {})", ret_val2.0, ret_val2.1);

    // You can use {:?} to print tuples
    println!("You can also return a tupple: {:?}", ret_val2);

    
/*
In Rust, the semicolon serves as a statement terminator. Here's a more detailed explanation:

1. Expressions vs. Statements:

   - An expression evaluates to a value.
   - A statement performs an action but does not produce a value.

2. Role of the Semicolon:

   - When you add a semicolon after an expression, you're telling Rust to evaluate that expression and then discard the result.
   - This act of evaluating and discarding turns the expression into a statement.

3. Semicolon Behavior:

   - Without a semicolon: `num1 + num2` is an expression that produces a value.
   - With a semicolon: `num1 + num2;` becomes a statement. It still computes the sum, but the result is immediately discarded.

4. Implicit Return:

   - Rust uses the value of the last expression in a block as the return value if there's no explicit `return` statement.
   - A statement (ended with a semicolon) doesn't produce a value, so there's nothing to return implicitly.

5. Unit Type:

   - When an expression is turned into a statement with a semicolon, it effectively returns the unit type `()`.
   - The unit type `()` in Rust is similar to `void` in other languages, representing "no value".

This behavior is intentional in Rust's design. It allows for a clear distinction between expressions that produce values and statements that perform actions without producing values. It also enables concise function bodies where the last expression can serve as the return value without needing an explicit `return` keyword.

Here's a small example to illustrate:


fn expression_example() -> i32 {
    5 + 3  // This is an expression, returns 8
}

fn statement_example() -> () {
    5 + 3;  // This is a statement, returns ()
}


In `expression_example`, the function returns `8`. In `statement_example`, the addition is performed but its result is discarded, and the function returns `()`.

Would you like me to clarify any part of this explanation further?

*/
}

fn max_expression(arg1: i32, arg2: i32) -> i32{
    if arg1 > arg2 {
        arg1
    } else {
        arg2
    }
}
fn max_statement(arg1: i32, arg2: i32) -> i32{
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

fn flow_control(){
    let max1 = max_expression(5, 4);
    println!("max_expression value is {}", max1);
    
    let max2 = max_statement(5, 4);
    println!("max_statement value is {}", max2);
    
    let oddity = nested_condition(5, 4);
    println!("oddity is {}", oddity);

    let (a_and_b, a_or_b) = and_or();
    println!("a or b is true: {}", a_or_b);
    println!("a and b is true: {}", a_and_b);

    match_enums();

    conditionless_loop();

    while_loop();

    for_iterator();

    FizzBuzz();
}

fn FizzBuzz(){
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

fn ownership(){
    { // stack allocated string
        // here s does not exist
        let s = "hello"; // from now on - s is valid, for the rest of the scope

    } // here s is no longer valid by default: println!("{}", s) <- will error
    
    { // heap allocated string
        let mut s = String::from("hello");
        s.push_str(", world");

    } // here s is no longer valid SINCE RUST calls drop

    // in order for Rust to be able to drop on block scope end - every value must have SINGLE OWNER

    {
        let mut s1 = String::from("hello"); // s1 is now the sole owner of the heap allocated string
        let s2 = s1; // ownership transfer! s2 is now the sole ownership of the heap allocated string

        // println!("{}", s1); <-- This will error!
        println!("{}", s2); // <-- This will work!
    }

    {
        let mut s1 = String::from("hello"); // s1 is now the sole owner of the heap allocated string
        let s2 = s1.clone(); // s2 is now the ownership of DIFFERENT heap allocated string (which is a clone of the original one)

        println!("{}", s1); // <-- This will work!
        println!("{}", s2); // <-- This will work!
    }

}

fn references(){
    {
        fn call_with_string(s: String){
            println!("call_with_string s: {}", s);
        }
    
        let s = String::from("Hello");
        println!("The value of s is {}", s);
    
        call_with_string(s);
        // println!("After sending s as parameter to a function, s is: {}", s);  <-- ERROR: borrow of moved value: `s`. value borrowed here after move
    }

    {
        fn give_ownership() -> String{
            let s = String::from("Hello from give_ownership");
            s
        }

        fn take_and_give_ownership(s:String) -> String{
            s
        }

        let s1 = give_ownership();
        println!("s1: {}", s1);

        let s2 = String::from("back and forth string ownership");
        let s3 = take_and_give_ownership(s2);
        println!("s3: {}", s3);

        // so, if we want to calculate the length of a string, we should return the length and the string itself!
        fn calc_string_length(s_to_calc: String) ->(String, usize){
            let len = s_to_calc.len();
            (s_to_calc, len)
        }
        let (s,len) = calc_string_length(String::from("hello"));
        println!("The length of {} is {}", s, len);
    }

    {
        // so, if we want to calculate the length of a string, we could send a "reference" to the string to avoid its being moved!
        fn calc_string_length(s_to_calc: &String) -> usize{
            let len = s_to_calc.len();
            len
        }

        let s = String::from("hello");
        let len = calc_string_length(&s);
        println!("The length of {} is {}", s, len);
    }

    {
        // If we want to modify a string, reference will not be enough. we need mutable reference!
        fn modify_string(s_to_modify: &mut String) -> &mut String{
            s_to_modify.push_str(" with pushed content");
            s_to_modify
        }

        let mut s = String::from("hello");
        let s_copy = s.clone();
        let modified_s = modify_string(&mut s);
        println!("The modificatin of {} is {}", s_copy, modified_s);
    }

    {
        let mut s = String::from("Hello");

        let s1 = &s;
        let s2 = &s;
        println!("s1: {}, s2: {}", s1, s2); //I can hold mutable reference OR shared immutable references. not both.
        let sMod = &mut s; // but I can do that "in turns". first I use some immutable shared references... and then I use single mutable one
        println!("s modified: {}", sMod);
    }
}

fn slices(){
    {
        let arr: [char; 5] = ['a', 'b', 'c', 'd', 'e'];
        let slice: &[char] = &arr[1..3];
        println!("{:?}", slice);

        let vec: Vec<i8> = vec![10, 20, 30, 40, 50];
        let vec_slice: &[i8] = &vec[1..3];
        println!("{:?}", vec_slice);

        let s_arr: [String; 5] = [String::from("alo"), String::from("blo"), String::from("clo"), String::from("dlo"), String::from("elo")];
        let s_slice: &[String] = &s_arr[1..3];
        println!("{:?}", s_slice);

        let s = String::from("Hello world");
        let hello: &str = &s[0..5];
        let world: &str = &s[6..11];
        println!("{}", hello);
        println!("{}", world);
    }

    {//Range shortcut
        let s = "Hello world";
        let hello: &str = &s[..5];
        let world: &str = &s[6..];
        let hello_world = &s[..];
        println!("{}", hello);
        println!("{}", world);
        println!("{}", hello_world);
    }

    // {//Ex.1 Given a string, silce it into its words
    //     //solution #1
    //     let s = String::from("Just a perfect day Drink sangria in the park And then later when it gets dark We go home");
    //     let mut word_start = 0;
    //     for (index, char) in s.chars().enumerate() {
    //         if char == ' ' {
    //             let new_word: &str = &s[word_start..index];
    //             println!("{}", new_word);
    //             word_start = index + 1;
    //         }
    //     }
    //     let new_word: &str = &s[word_start..];
    //     println!("{}", new_word);
    // }

    // {//Ex.2 Given a string, silce it into its words
    //     //solution #2 - no slices
    //     let mut s = String::from("Just a perfect day Drink sangria in the park And then later when it gets dark We go home");
    //     let mut has_chars = s.len();

    //     while s.len() > 0 {
    //         let mut first_word_index = first_word(&s);
    //         if s.len() > first_word_index {// still have spaces
    //             first_word_index += 1;
    //         } 
    //         let removed: String = s.drain(..first_word_index).collect();
    //         println!("{}", removed);
    //     }

    //     fn first_word(s: &String) -> usize{

    //         let bytes = s.as_bytes();

    //         for (i, &item) in bytes.iter().enumerate() {
    //             if item == b' ' {
    //                 return i;
    //             }
    //         }

    //         return s.len()
    //     }


    // }

    // {// slices with string literals
    //     let s1 = "another string";
    //     let s2 = String::from("another string");

    //     let mut first_word = first_word_fn(s1); // string literals ARE SLICES;
    //     println!("{}", first_word);

    //     first_word = first_word_fn(&s2);
    //     println!("{}", first_word);

    //     fn first_word_fn(s: &str) -> &str{
    //         let bytes = s.as_bytes();

    //         for (i, &item) in bytes.iter().enumerate() {
    //             if item == b' ' {
    //                 return &s[0..i];
    //             }
    //         }

    //         return &s[..]
    //     }


    // }

   


}

fn structs(){
    #[derive(Debug)]
    struct User {
        name: String,
        email: String,
        is_active: bool,
        age: u8
    }

    let user1 = User {
        name: String::from("John Doe"),
        email: String::from("doe@mail.com"),
        is_active: true,
        age: 25
    };

    println!("User1 name: {} \nEmail: {} \nIs active: {} \nAge: {}", user1.name, user1.email, user1.is_active, user1.age);
    println!("{:?}", user1);

    //structs are immutable by default, so:
    // user1.name = String::from("Othe name") will Error

    let mut user2 =  User {
        name: String::from("Mutable user"),
        email: String::from("mut@mail.com"),
        is_active: true,
        age: 25
    };

    // now we can mutate:
    user2.age = 5;
    println!("{:?}", user2);
}


fn main(){
    // vars();
    // data_types();
    // another_function(42, 'a', "hello");
    // statements_vs_expressions()
    // flow_control()
    // ownership()
    // references()
    // slices();

    structs()
}
