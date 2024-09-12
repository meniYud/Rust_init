/*
Source: https://www.youtube.com/watch?v=2XwP1U0XL0E&list=PLPoSdR46FgI412aItyJhj2bF66cudB6Qs&index=22

In this video, we dive into error handling in Rust, focusing on unrecoverable errors using the panic! macro. 

We start by explaining the basics of error handling in Rust and then move on to how the panic! macro is used for unrecoverable errors.
You'll learn how to use a panic backtrace to debug issues and understand how a panic! call can indicate a bug in your code. 

We also compare Rust's approach to error handling with C's handling of undefined behavior. Finally, we wrap up with a brief conclusion.

*/

pub fn panicing(){
    // run with: RUST_BACKTRACE=full cargo run
    //Unrecoverable errors
    panic!("crash and burn");

    let v = vec![1,2,3];

    v[99]; // <-- panic
}