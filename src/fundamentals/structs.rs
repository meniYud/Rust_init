/*
Source: https://www.youtube.com/watch?v=PCjuO-Bv5FI&list=PLPoSdR46FgI412aItyJhj2bF66cudB6Qs&index=9

Structs are a way to create more complex data types in Rust.
They are similar to tuples, but with a few differences.
You can name the fields of a struct (and you can also define methods for structs).
You can also make a struct mutable by using the mut keyword. 

You can create a function that returns a struct instance.
You can create a new instance of a struct using another instance with the struct update syntax.
You can define tuple structs and unit-like structs.

*/

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
struct Point3d(i32, i32, i32);
pub fn structs(){
    
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

pub fn struct_factory() {
    fn build_user(name: String, email: String, age: u8) -> User{
        User {
            name,
            email,
            is_active: true,
            age
        }
    }

    let user1 = build_user(String::from("First"), String::from("john@email.com"), 18);
    let user2 = build_user(String::from("Second"), user1.email.clone(), user1.age);

    println!("{:?}", user1);
    println!("{:?}", user2); // user2 had to clone user1.email since he has no ownership over the string
}

pub fn tuples_struct(){
    let black = Color(0,0,0);
    let origin = Point3d(0,0,0);

    println!("R: {}, G: {}, B: {}", black.0, black.1, black.2);
    println!("X: {}, Y: {}, Z: {}", origin.0, origin.1, origin.2);

    //unit-like structs:
    #[derive(Debug)]
    struct Car;
    println!("Unit-like struct: {:?}", Car);
}

/*
Source: https://www.youtube.com/watch?v=mgK5LezkHl8&list=PLPoSdR46FgI412aItyJhj2bF66cudB6Qs&index=10
Source: https://www.youtube.com/watch?v=94iSHoKmPmY&list=PLPoSdR46FgI412aItyJhj2bF66cudB6Qs&index=11


To understand when we might want to use structs, let’s write a program that calculates the area of a rectangle.
We’ll start by using single variables and then refactor the program until we’re using structs instead.


Methods are similar to functions: we declare them with the fn keyword and a name,
they can have parameters and a return value, and they contain some code that’s run when the method is called from somewhere else.
Unlike functions, methods are defined within the context of a struct, and their first parameter is always self,
which represents the instance of the struct the method is being called on.
*/

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
pub fn structs_in_use(){
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
