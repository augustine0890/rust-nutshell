#![allow(dead_code)]
use std::fs;
use std::io;

/*
- Propagating errors: allow higher-level code to better handle potential errors by providing more information and context
- Result<T, E> enum: an enum that represents the result of an operation that can either succeed or fail.
 */
fn read_and_combine(f1: &str, f2: &str) -> Result<String, io::Error> {
    let mut s1 = fs::read_to_string(f1)?;

    let s2 = match fs::read_to_string(f2) {
        Ok(s) => s,
        Err(e) => return Err(e),
    };

    s1.push('\n');
    s1.push_str(&s2);
    Ok(s1)
}

pub fn run() {
    let combine_res = read_and_combine("planets.txt", "s2_planets.txt");
    // let combine_res = read_and_combine("planets.txt", "moonwalkers.txt");
    match combine_res {
        Ok(res) => println!("The conmbined results: \n{}", res),
        Err(e) => println!("There was an error: {}", e),
    }

    let result = fs::read_to_string("file.txt");
    let contents = match result {
        Ok(message) => message,
        Err(error) => match error.kind() {
            io::ErrorKind::NotFound => format!("Nobody knows the 'file.txt' (error)"),
            io::ErrorKind::PermissionDenied => String::from("Permission denied"),
            _ => panic!("{}", error),
        },
    };

    println!("The contents: {}", contents);
}
