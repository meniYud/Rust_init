mod vars;
use vars::vars;

mod data_types;
use data_types::data_types;

mod statements_vs_expressions;
use statements_vs_expressions::statements_vs_expressions;

mod flow_control;
use flow_control::flow_control;

mod ownership;
use ownership::ownership;
use ownership::references;
use ownership::slices;

mod matcher;
use matcher::matcher;
use matcher::if_let;








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
    // flow_control()

    // ownership()
    // references()
    slices();
    
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
