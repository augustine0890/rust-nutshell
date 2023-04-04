use std::fs;
use std::io;

pub fn run() {
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
