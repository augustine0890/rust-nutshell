#![allow(dead_code)]
use std::io;
use std::num::ParseIntError;
// use rand::random;
use rand::prelude::*;

/*
- Cretes: a collection of Rust source code files.
- The Prelude automatically important a variety of commonly used things for every Rust program to use.
 */
pub fn run() {
    let random_number = random::<f64>();
    println!("Random number is: {}", random_number);

    let rand_num = thread_rng().gen_range(1..11);
    println!("Random number is: {}", rand_num);

    // Guessing game
    guess_number();
}

fn guess_number() {
    let secret_number = rand::thread_rng().gen_range(0..100);
    println!("Enter your guess number (0-99). You have six chances to guess the number.");
    const MAX_GUESSES: u32 = 6;

    for attempt in 1..=MAX_GUESSES {
        let remaining_attempts = MAX_GUESSES - attempt;
        match guess_again() {
            Ok(value) => match value {
                value if value < secret_number => {
                    println!(
                        "Your number is too low. Keep guessing! Attempts remaining: {}",
                        remaining_attempts
                    );
                }
                value if value > secret_number => {
                    println!(
                        "Your number is too high. Keep guessing! Attempts remaining: {}",
                        remaining_attempts
                    );
                }
                _ => {
                    println!("Correct! The secret number is: {}", secret_number);
                    return;
                }
            },

            Err(_) => {
                println!(
                    "\nFailed to parse input. Guess again. Attempts remaining: {}",
                    remaining_attempts
                );
            }
        }
    }

    println!(
        "Sorry! You have used all of your guessing times. The secret number is: {}",
        secret_number
    );
}

fn guess_again() -> Result<i32, ParseIntError> {
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read input line.");
    println!("Your answer is: {}", guess);

    guess.trim().parse::<i32>()
    // let number = guess.trim().parse::<i32>()?;
    // Ok(number)
}
