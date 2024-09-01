
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

fn square(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i*i)
    }
}

pub fn matcher(){
    {
        let value = value_in_cents(Coin::Nickel);
        println!("The value is: {}", value);
    
        let coin_with_rarity = value_in_cents(Coin::Quarter(Rarity::Epic));
        println!("The value of the coin is: {}", coin_with_rarity);
    }

    {
        let mult1 = Some(5);
        let mult2 = None;
        let mut sq = square(mult1);
        println!("The square of {:?} is {:?}", mult1, sq);
        sq = square(mult2);
        println!("The square of {:?} is {:?}", mult2, sq);
    }

    {
        let dice_roll = 9;

        match dice_roll {
            1 => println!("You rolled one"),
            2 => println!("You rolled two"),
            3 => println!("You rolled three"),
            4 => println!("You rolled four"),
            5 => println!("You rolled five"),
            _other => println!("You rolled something else!"),
        }

    }

    {

    }
}

pub fn if_let(){
    {
        let config_max = Some(100);
        match config_max {
            Some(max) => println!("The max is configuted to be: {}", max),
            _ => ()
        }// match is to much here... we can use if-let statement

        if let Some(max) = config_max {
            println!("The max is configuted to be: {}", max)
        }
    }

    {
        fn rarity_printer(coin: Coin){
            if let Coin::Quarter(rarity) = coin {
                println!("The rarity of the coin is: {:?}", rarity)
            } else {
                println!("No rarity to the given coin");
            }
        }
        let rare_coin1 = Coin::Dime;
        rarity_printer(rare_coin1);
        let rare_coin2 = Coin::Quarter(Rarity::Legendary);
        rarity_printer(rare_coin2);
        
    }
}