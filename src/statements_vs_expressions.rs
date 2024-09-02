
fn sum(num1: i32, num2: i32) -> i32 {
   num1 + num2
}
fn sum_and_sub(num1: i32, num2: i32) -> (i32, i32) {
   return (num1 + num2, num1 - num2);
   println!("This will not be printed");
}

pub fn statements_vs_expressions(){
    let x = 42; //statement (does not return value)
    let z = 9 + x; //The RHS is an expression (returns a value)

    // block scopes are expression
    let y = {
        let x = 3;
        
        x + 1
        // NOT x + 1;
    };
    println!("The returned value of the scope is {}", y);
    println!("Note: if you add semi;colons at the end of line 221, the block scope will become a statement rather than expression");

    //Functions are also expressions
    println!("The returned value is {}", sum(5,7));
    println!("Note: if you add semi;colons at the end of line 206, the block scope will become a statement rather than expression");

    let ret_val2 = sum_and_sub(5,7);
    println!("You can also return a tupple: ({}, {})", ret_val2.0, ret_val2.1);

    // You can use {:?} to print tuples
    println!("You can also return a tupple: {:?}", ret_val2);

    
/*
In Rust, the semicolon serves as a statement terminator. Here's a more detailed explanation:

1. Expressions vs. Statements:

   - An expression evaluates to a value.
   - A statement performs an action but does not produce a value.

2. Role of the Semicolon:

   - When you add a semicolon after an expression, you're telling Rust to evaluate that expression and then discard the result.
   - This act of evaluating and discarding turns the expression into a statement.

3. Semicolon Behavior:

   - Without a semicolon: `num1 + num2` is an expression that produces a value.
   - With a semicolon: `num1 + num2;` becomes a statement. It still computes the sum, but the result is immediately discarded.

4. Implicit Return:

   - Rust uses the value of the last expression in a block as the return value if there's no explicit `return` statement.
   - A statement (ended with a semicolon) doesn't produce a value, so there's nothing to return implicitly.

5. Unit Type:

   - When an expression is turned into a statement with a semicolon, it effectively returns the unit type `()`.
   - The unit type `()` in Rust is similar to `void` in other languages, representing "no value".

This behavior is intentional in Rust's design. It allows for a clear distinction between expressions that produce values and statements that perform actions without producing values. It also enables concise function bodies where the last expression can serve as the return value without needing an explicit `return` keyword.

Here's a small example to illustrate:


fn expression_example() -> i32 {
    5 + 3  // This is an expression, returns 8
}

fn statement_example() -> () {
    5 + 3;  // This is a statement, returns ()
}


In `expression_example`, the function returns `8`. In `statement_example`, the addition is performed but its result is discarded, and the function returns `()`.

Would you like me to clarify any part of this explanation further?

*/
}

