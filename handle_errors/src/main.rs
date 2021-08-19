use rand::prelude::*;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("I'm thinking of a number between 1 and 100...");
    println!("Guess the number:");
    loop {
        let guess: u32;
        loop {
            let mut guess_string = String::new();
            match io::stdin().read_line(&mut guess_string) {
                Err(_) => {
                    println!("Failed to read input line.");
                    continue;
                }
                _ => {}
            }
            match guess_string.trim().parse::<u32>() {
                Ok(n) => {
                    guess = n;
                    break;
                }
                Err(_) => println!("Please enter a whole number:"),
            }
        }
        if guess > secret_number {
            println!("\n{} is too high! Guess lower:", guess);
        } else if guess < secret_number {
            println!("\n{} is too low! Guess higher:", guess);
        } else {
            println!("\nYou got it! The secret number was {}.", secret_number);
            break;
        }
    }
}
