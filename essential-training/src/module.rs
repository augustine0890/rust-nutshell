use std::io;
// use rand::random;
use rand::prelude::*;

/*
Cretes: a collection of Rust source code files
 */
pub fn run() {
    let random_number = random::<f64>();
    println!("Random number is: {}", random_number);

    let rand_num = thread_rng().gen_range(1..11);
    println!("Random number is: {}", rand_num);

    guess_number();
}

fn guess_number() {
    let secret_number = rand::thread_rng().gen_range(0..100);
    println!("Enter the your number (0-99):");
    
    loop {
        let number = guess_again();

        if secret_number > number {
            println!("Your number is too low. Keep guessing!")
        } else if secret_number < number{
            println!("Your number is too high. Keep guessing!")
        } else {
            println!("Correct! The number is: {}", secret_number);
            break;
        }
    }
}

fn guess_again() -> i32 {
    let mut guess_number = String::new();
    io::stdin().read_line(&mut guess_number).expect("Failed to read input line.");
    println!("Your answer is: {}", guess_number);
    guess_number.trim().parse::<i32>().expect("Failed to parse the guess.")
}