/*
Source: https://www.youtube.com/watch?v=NBFH3BnCIvU&list=PLPoSdR46FgI412aItyJhj2bF66cudB6Qs&index=23


By the end of this video, you will learn how to handle recoverable errors in Rust using the Result type.
We start with a basic example of recoverable errors and demonstrate how to use the match statement to handle different error cases. 

You'll see how to manage results with if-else statements and get familiar with shortcuts like unwrap and expect. 

We also cover error propagation and introduce the ? operator for more streamlined error handling.
Discover where the ? operator can be used, including its application in the main function. 

*/

use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};

pub fn result_enum(){
    // {
    //     let greeting_from_file = File::open("hello.txt");
    //     let greeting_file = match greeting_from_file {
    //         Ok(file) => file,
    //         Err(error) => match error.kind() {
    //             ErrorKind::NotFound => match File::create("hello.txt") {
    //                 Ok(fc) => fc,
    //                 Err(error) => panic!("Unable to create the file: {:?}", error),
    //             },
    //             other_error => {
    //                 panic!("File found, but other error occured: {:?}", other_error);
    //             }
    //         }
    //     };
    // }

    // {
    //     File::open("hello.txt").unwrap_or_else(|error| {
    //         if error.kind() == ErrorKind::NotFound {
    //             File::create("hello.txt").unwrap_or_else(|error| {
    //                 panic!("Unable to create the file: {:?}", error);
    //             })
    //         } else {
    //             panic!("File found, but other error occured: {:?}", error);
    //         }
    //     });
        
    // }

    // {
    //     let greeting_from_file = File::open("hello.txt").expect("****   Nein Nein Nein Nein   ****");
    // }
    

    //     fn read_from_file() -> Result<String, io::Error> {
    //         let f = File::open("tail.txt");

    //         let mut f = match f {
    //             Ok(file) => file,
    //             Err(err) => return Err(err)
    //         };

    //         let mut s = String::new();

    //         match f.read_to_string(&mut s){
    //             Ok(_) => Ok(s),
    //             Err(e) => return Err(e)
    //         }

        // fn read_from_file() -> Result<String, io::Error> {
        //     let mut file = File::open("tail.txt")?;
        //     let mut content = String::new();

        //     file.read_to_string(&mut content)?;
        //     Ok(content)
        // }

        fn read_from_file() -> Result<String, io::Error> {
            let mut content = String::new();
            File::open("tail.txt")?.read_to_string(&mut content)?;

            Ok(content)
        }

        
        let res = read_from_file();
        match res {
            Ok(s) => println!("Success: {}", s),
            Err(e) => println!("Failed: {}", e)
        }

    
}