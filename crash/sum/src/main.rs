use std::io;
use std::process;

fn main() {
    let result = calculate_sum();
    println!("The sum of the inputs: {}", result);

    let res = divide(5, 0);
    // let value = res.expect("Failed to divide");
    // println!("Result: {}", value);
    match res {
        Ok(value) => println!("The result of the division is {}", value),
        Err(err) => {
            println!("Error: {}", err);
            process::exit(1)
        }
    }
}

fn calculate_sum() -> u32 {
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
    // let b: u32 = second.trim().parse().unwrap();
    let b: u32 = second
        .trim()
        .parse()
        .expect("Error parsing: may not a valid number");

    sum(a, b)
}

fn sum(a: u32, b: u32) -> u32 {
    a + b
}

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        return Err(String::from("Cannot divide by zero"));
    }
    Ok(a / b)
}
