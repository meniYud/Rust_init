/*
Source: https://www.youtube.com/watch?v=9VBLOwmNE1g&list=PLPoSdR46FgI412aItyJhj2bF66cudB6Qs&index=6

In this lesson, we will cover the following topics:
- What is ownership?
- The `Stack` and the `Heap`
- Ownership rules
- Variable scope
- The `String` type
- Memory allocation and the `Drop` function
- The `Move` trait
- The `Clone` trait
- The `Copy` trait
- Final Recap
*/

pub fn ownership(){
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

/*
Source: https://www.youtube.com/watch?v=Q_0yoX07Fhs&list=PLPoSdR46FgI412aItyJhj2bF66cudB6Qs&index=7

In this lesson, we will see:
- Ownership and Functions
- Return Values and Scope
- Introduction to References and Borrowing
- Mutable References
- Multiple Mutable References
- Mutable and Immutable References
- Dangling references
- References Rules
*/

pub fn references(){
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

/*
Source: https://www.youtube.com/watch?v=dKymZbFp0ZQ&list=PLPoSdR46FgI412aItyJhj2bF66cudB6Qs&index=8


Rust has a built-in type called Slice that is used to reference a contiguous sequence of elements in a collection. 
Slices are a very important concept in Rust, and they are used extensively in the standard library. 
In this lesson, we will explore the Slice type and how it is used in Rust.

*/
pub fn slices(){
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
