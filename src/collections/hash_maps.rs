/*
Source: https://www.youtube.com/watch?v=-yXcVNqo0DQ&list=PLPoSdR46FgI412aItyJhj2bF66cudB6Qs&index=21

This comprehensive beginner's tutorial teaches you how to store keys with associated values using Hash Maps in Rust.
We'll cover importing Hash Maps from the standard library, updating values based on the old value, and more 

It's perfect for those new to Rust and looking to deepen their understanding of this powerful language feature.
*/
use std::{collections::HashMap, iter::Copied};

pub fn hash_maps_operations(){
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("Scores: {:?}", scores);
    

    let team1_name = String::from("Blue");
    let team2_name = String::from("Red");
    let team1_score = scores.get(&team1_name).copied().unwrap_or(0);
    let team2_score = scores.get(&team2_name).copied().unwrap_or(0);
    println!("{} team score: {}", team1_name, team1_score);
    println!("{} team score: {}", team2_name, team2_score);

    //Iterations
    let mut languages = HashMap::new();

    languages.insert(String::from("Rust"), 1);
    languages.insert(String::from("Python"), 2);
    languages.insert(String::from("Java"), 3);
    languages.insert(String::from("C++"), 4);
    languages.insert(String::from("Javascript"), 5);

    for (key, value) in &languages {
        println!("{}: {}", key, value)
    }

    //HashMaps and ownership
    let mut map2 = HashMap::new();

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    map2.insert(&field_name, &field_value);

    println!("{:?}", map2);
    println!("field_name: {}", field_name);

    let mut map3 = HashMap::new();

    let text = String::from("Hello, world");
    let number = 10;

    map3.insert(&text, number);// String does not implement the "copy" trait so we must use borrowed value, but number does imlement the "copy" trait

    println!("{:?}", map3);
    println!("text: {}", text);
    println!("number: {}", number);
}