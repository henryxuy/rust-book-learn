/**
 * By default, variables are immutable in rust
 * 
 */

fn main() {
    // 1. immutable, mutable variables
    let mut x = 5;
    // let x = 5;
    println!("The value of x is: {}", x);
    // compiler would check immutability of immutable variables
    x = 6;
    println!("The value of x is: {}", x);

    // 2. consts
    // (1) cannot use `mut`. Must specify type
    // (2) can be declared in any scope
    // (3) constants may be set only to a constant expression, not the result of a value that could only be computed at runtime
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // 3. shadow
    // Note: Here y is immutable
    let y = 5;
    let y = y + 1;          // shadow y with 6
    {   
        let y = y * 2;      // shadow y with 12, in the inner scope only
        println!("The value of y in the inner scope is: {}", y);
    }

    // back to the outer scope, y is recovered to 6
    println!("The value of y is: {}", y);


    // Use different types to shadow is allowed
    let spaces = "   ";
    // spaces = spaces.len()            // not allowed
    let spaces = spaces.len();
    
    println!("The value of spaces is: {}", spaces);
}
