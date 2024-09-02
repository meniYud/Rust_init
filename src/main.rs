mod vars;
use vars::vars;

mod data_types;
use data_types::data_types;

mod statements_vs_expressions;
use statements_vs_expressions::statements_vs_expressions;

mod flow_control;
use flow_control::flow_control;

mod matcher;
use matcher::matcher;
use matcher::if_let;






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

#[derive(Debug)]
struct User {
    name: String,
    email: String,
    is_active: bool,
    age: u8
}

#[derive(Debug)]
struct Color(i32, i32, i32);

#[derive(Debug)]
struct Point_3d(i32, i32, i32);
fn structs(){
    
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

fn struct_factory() {
    fn build_user(name: String, email: String, age: u8) -> User{
        User {
            name,
            email,
            is_active: true,
            age
        }
    };

    let user1 = build_user(String::from("First"), String::from("john@email.com"), 18);
    let user2 = build_user(String::from("Second"), user1.email.clone(), user1.age);

    println!("{:?}", user1);
    println!("{:?}", user2); // user2 had to clone user1.email since he has no ownership over the string
}

fn tuples_struct(){
    let black = Color(0,0,0);
    let origin = Point_3d(0,0,0);

    println!("R: {}, G: {}, B: {}", black.0, black.1, black.2);
    println!("X: {}, Y: {}, Z: {}", origin.0, origin.1, origin.2);

    //unit-like structs:
    #[derive(Debug)]
    struct Car;
    println!("Unit-like struct: {:?}", Car);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
//impl block is used to define methods in the context of some struct
impl Rectangle{
    fn calculate_area(&self) -> u32{
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        (self.width > other.width && self.height > other.height) ||
        (self.width > other.height && self.height > other.width)
    }

    // method with the same name as a field
    fn width(&self) -> bool {
        self.width > 0
    }
    fn height(&self) -> bool {
        self.height > 0
    }
    //we can use these kind of methods to create getters

    //associated function to define square
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size
        }
    }
}
fn structs_in_use(){
    { // without structs
        fn calculate_rect_area(width: u32, height: u32) -> u32{
            width * height
        }
        let width = 30;
        let height = 50;
        let area = calculate_rect_area(width, height);
        println!("The area is: {}", area);
    }

    { // with structs (We could also do that with tuple struct)
        
        fn calculate_rect_area(rect: &Rectangle) -> u32{
            rect.width * rect.height
        }

        let rect1 = Rectangle{
            width: 30,
            height: 50
        };

        let area = calculate_rect_area(&rect1);
        println!("The Rectangle: {:#?}", rect1);
        println!("The area of the rectangle is: {}", area);
    }

    {// using struct's method (impl block)
        let rect1 = Rectangle{
            width: 30,
            height: 50
        };
        let rect2 = Rectangle{
            width: 0,
            height: 30
        };

        if(rect2.width() && rect2.height()){
            println!("Using the method, the area is: {}", rect2.calculate_area());
        } else {
            println!("ERROR!\nThe width is: {}\nThe height is: {}", rect2.width, rect2.height);
        }

        println!("Renct 1 can hold rect 2: {}", rect1.can_hold(&rect2))

    }

    {
        let square = Rectangle::square(20);
        println!("The size of our square is: {}", square.calculate_area())
    }

}

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6
}

#[derive(Debug)]
struct Host {
    host_name: String,
    ip: Option<String>,
    ip_type: Option<IpAddrKind>,
}

impl Host {
    fn set_ip(&mut self, ip: String){
        let ip_parts: Vec<&str> = ip.split('.').collect();
        let is_v4 = ip_parts.len() == 4;
        self.ip = Some(ip);
        self.ip_type = if is_v4 {Some(IpAddrKind::V4)} else {Some(IpAddrKind::V6)};
    }
    
}

#[derive(Debug)]
enum IpAddrType {
    V4(u8, u8, u8, u8),
    V6(String)
}

fn enums(){
    fn ip_addr_printer(ip: IpAddrKind){
        println!("Your IP version is: {:?}", ip);
    }
    {
        let four = IpAddrKind::V4;
        let six = IpAddrKind::V6;

        ip_addr_printer(four);
        ip_addr_printer(six);
    }

    {
        let mut my_host = Host {
            host_name: String::from("personal_laptop"),
            ip: None,
            ip_type: None
        };

        my_host.set_ip(String::from("1.2.3.4"));
        println!("My host details: {:?}", my_host);
    }

    {
        let home = IpAddrType::V4(127,0,0,1);
        println!("My home address is: {:?}", home);

        let loopback = IpAddrType::V6(String::from("::1"));
        println!("My loopback address is: {:?}", loopback);
    }

}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self){
        println!("{:?}", self);
    }
}

fn enum_methods(){
    let writer = Message::Write(String::from("Hello"));
    let mover = Message::Move { x: (3014), y: (151) };
    let painter = Message::ChangeColor(200, 255, 0);
    let quiter = Message::Quit;

    writer.call();
    mover.call();
    painter.call();
    quiter.call();
}

fn optionals(){
    let x: i8 = 5;
    let y: Option<i8> = Some(4);

    // let sum = x + y;  <- Error: cannot add `Option<{integer}>` to `{integer}`

    let sum = x + y.unwrap();
    println!("The sum is: {}", sum);
}

fn main(){
    // vars();
    // data_types();
    // statements_vs_expressions()
    
    flow_control()
    // ownership()
    // references()
    // slices();
    // structs()
    // struct_factory()
    // tuples_struct()
    // structs_in_use()
    // enums()
    // enum_methods()
    // optionals()
    // matcher();
    // if_let()
}
