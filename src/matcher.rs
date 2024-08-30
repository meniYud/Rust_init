
#[derive(Debug)]
pub enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}
pub fn matcher(){
    fn value_in_cents(coin: Coin) -> usize {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }

    let value = value_in_cents(Coin::Nickel);
    println!("The value is: {}", value);
}