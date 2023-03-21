use std::io;

fn main() {
    println!("Please enter your name: ");

    let mut name = String::new();
    match io::stdin().read_line(&mut name) {
        // `n` is the number of bytes read from input
        Ok(n) => {
            println!("Read {} bytes from input: {}", n, name);
        }
        Err(error) => {
            println!("Error reading input: {}", error);
        }
    }

    println!("Hi, {}", name);
}
