/*
Source: https://www.youtube.com/watch?v=NyqJp5M3hRE&list=PLPoSdR46FgI412aItyJhj2bF66cudB6Qs&index=3
Rust is a statically typed language, which means that it must know the types of all variables at compile time. 

The compiler can usually infer what type we want to use based on the value and how we use it.
In cases when many types are possible, we must add a type annotation.

In this lesson, we will cover the basic data types in Rust.

We will talk about:
- Scalar types
- Compound types
- Custom types
*/
pub fn data_types() {
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