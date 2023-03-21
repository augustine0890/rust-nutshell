use std::io;

fn main() {
    println!("Please enter a first number: ");
    let mut first = String::new();
    match io::stdin().read_line(&mut first) {
        Ok(_) => println!("The first number is {}", first),
        Err(error) => println!("Error reading from stdin: {}", error),
    }
    // unwrap() will panic with a default error message and should only be used for quick developments
    let a: u32 = first.trim().parse().unwrap();

    println!("Please enter a second number: ");
    let mut second = String::new();
    match io::stdin().read_line(&mut second) {
        Ok(_) => println!("The second number is {}", second),
        Err(error) => println!("Error reading from stdin: {}", error),
    }
    let b: u32 = second.trim().parse().unwrap();

    let result = sum(a, b);
    println!("{} + {} = {}", a, b, result);
}

fn sum(a: u32, b: u32) -> u32 {
    a + b
}
