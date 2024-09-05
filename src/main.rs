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

mod structs;
use structs::structs;
use structs::struct_factory;
use structs::tuples_struct;
use structs::structs_in_use;

mod matcher;
use matcher::matcher;
use matcher::if_let;









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
    // slices();

    // structs()
    // struct_factory()
    // tuples_struct();
    structs_in_use()

    // enums()
    // enum_methods()
    // optionals()
    // matcher();
    // if_let()
}
