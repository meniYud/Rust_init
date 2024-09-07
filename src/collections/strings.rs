/*
Source: https://www.youtube.com/watch?v=GK9Iz_ihmV8&list=PLPoSdR46FgI412aItyJhj2bF66cudB6Qs&index=20

Dive into the world of Strings and UTF-8 in Rust with our detailed tutorial.
Understand how Rust handles strings as collections of bytes,
create and update strings, and learn advanced techniques like concatenation, slicing, and iteration.

This video covers everything from the basics to advanced string manipulation.
*/

pub fn string_operations(){
    //costructing an empty string
    let mut s = String::new();

    //converting &str to String
    let s = "this string was of type &str - but its now a String".to_string();

    println!("s: {}", s);

    //"from" initializer
    let s = String::from("initialized using \"from\"");

    println!("s: {}", s);

    //update a string
    let mut s = String::from("first");
    s.push_str(", second");

    println!("updated string: {}", s);

    s.push('!');
    println!("updated with single char: {}", s);

    

}