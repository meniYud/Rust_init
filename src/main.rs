fn vars() {
    {
        //types
        let mut small_int = 21648;
        println!("The value of small_int is {}, which is i32", small_int);
    
        let big_int: i64 = 42587621648;
        println!("The value of big_int is {}, which is i64", big_int);
    
        small_int = 5246;
        println!("small_int value is now {}. It is a mutable thanks to the 'mut' operator", small_int);
    }
    
    {
        //shadowing
    
        let x = 5;
        println!("The value of x is {}", x);
    
        {
            let x = 6;
            println!("The value of x inside this scope is {}", x);
        }
    
        let x = x + 2;
        println!("Back to the main scope, the value of x is now {}", x);
    }

    {
        //mutability meet types
        let a = 5;
        println!("Before mutation, a is a number with value {}", a);

        let a = "Holla";
        println!("After mutation, a is a string with value {}", a);

        let mut b = 5;
        println!("Since b is mutable variable - it cannot switch types");

        // b = "NO";
    }

    {
        //constants
        const MAX_POINTS:i32 = 100_000;
        println!("The value of MAX_POINTS must be initialized as part of declaration {}", MAX_POINTS);

        {
            const MAX_POINTS:i32 = 100_001;
            println!("The value of MAX_POINTS can be override in an inner scope {}", MAX_POINTS);
        }
    }


}







fn main(){
    vars()
}
