use std::io;


fn main() {
    println!("Guess the number!");

    println!("Please input your guess");

    // In rust, variables are immutable by default. If you want to make it mutable, need to add 'mut' keyword
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess) // Reference (&) is immutable by default, Even for a mutable variable. Need & mut x
        .expect("Failed to read line");

    println!("You guessed: {}", guess)
}
