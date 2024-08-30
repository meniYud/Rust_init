
#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(Rarity),
}

#[derive(Debug)]
enum Rarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary
}

pub fn matcher(){
    fn value_in_cents(coin: Coin) -> usize {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(rarity) => {
                println!("The rarity of this coin is: {:?}", rarity);
                25
            },
        }
    }

    let value = value_in_cents(Coin::Nickel);
    println!("The value is: {}", value);

    let coin_with_rarity = value_in_cents(Coin::Quarter(Rarity::Epic));
    println!("The value of the coin is: {}", coin_with_rarity);
}