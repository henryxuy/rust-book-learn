use std::io;
use std::cmp::Ordering;
use rand::Rng;      
// `use`: bring defined items into scope explicitly

// The Rng trait defines methods that random number generators implement, and this trait must be in scope for us to use those methods
// will be introduced more in ch10

// Can use: `cargo doc --open` to generate doc for all used cartes and open it

// `fn`: define a function
fn main() {
    println!("Guess the number!");

    // rand::thread_rng(): lazily-initialized thread-local random number generator
    // [start..end): incl. state, but excl. end
    // `let`: create a variable with some name, and bind the value to the variable name
    let secret_number = rand::thread_rng().gen_range(1..101);

    // println!("The secret number is: {}", secret_number);

    // use `loop` keyword to create a loop
    loop {
        println!("Please input your guess");

        // In rust, variables are immutable by default. If you want to make it mutable, need to add 'mut' keyword
        let mut guess: String = String::new();

        // Result<T, Error>.expect("msg here")
        io::stdin()
            .read_line(&mut guess) // Reference (&) is immutable by default, Even for a mutable variable. Need & mut x
            .expect("Failed to read line");

        // similar to
        // match io::stdin()
        //     .read_line(&mut guess) {
        //         Ok(_) => {}
        //         Err(_) => println!("Failed to read line"),
        // }


        println!("You guessed: {}", guess);

        // rust allows us to use a new value to shadow the old type bound to that variable name
        // Here, we use guess: u32 to shadow guess: String
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        let guess: u32 = match guess.trim().parse() {
            // use match instead of expect: match Result<F, F::Err>
            Ok(num) => num,                     
            Err(_) => continue,
        };


        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => {println!("Too big!");},
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }

}
