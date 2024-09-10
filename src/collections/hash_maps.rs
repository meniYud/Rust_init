/*
Source: https://www.youtube.com/watch?v=-yXcVNqo0DQ&list=PLPoSdR46FgI412aItyJhj2bF66cudB6Qs&index=21

This comprehensive beginner's tutorial teaches you how to store keys with associated values using Hash Maps in Rust.
We'll cover importing Hash Maps from the standard library, updating values based on the old value, and more 

It's perfect for those new to Rust and looking to deepen their understanding of this powerful language feature.
*/
use std::collections::HashMap;

pub fn hash_maps_operations(){
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("Scores: {:?}", scores);
    
}