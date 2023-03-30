#![allow(dead_code)]
use std::io;
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
    // guess_number();
}

fn guess_number() {
    let secret_number = rand::thread_rng().gen_range(0..100);
    println!("Enter your guess number (0-99). You have six chances to guess the number.");
    let mut guessing_times = 6;

    let mut guessed_correctly = false;
    while guessing_times > 0 {
        let number = guess_again();
        guessing_times -= 1;
        if secret_number > number {
            println!("Your number is too low. Keep guessing!")
        } else if secret_number < number {
            println!("Your number is too high. Keep guessing!")
        } else {
            println!("Correct! The secret number is: {}", secret_number);
            guessed_correctly = true;
            break;
        }
    }

    if !guessed_correctly {
        println!(
            "Sorry! You have used all of your guessing times. The secret number is: {}",
            secret_number
        );
    }
}

fn guess_again() -> i32 {
    let mut guess_number = String::new();
    io::stdin()
        .read_line(&mut guess_number)
        .expect("Failed to read input line.");
    println!("Your answer is: {}", guess_number);
    guess_number
        .trim()
        .parse::<i32>()
        .expect("Failed to parse the guess.")
}