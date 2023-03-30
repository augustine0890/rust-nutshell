use std::io::prelude::*;
use std::time::Duration;
use std::{env, fs, thread};

pub fn run() {
    for (index, argument) in env::args().enumerate() {
        println!("argument {} is {}", index, argument);
    }

    read_file();

    write_file();
}

fn read_file() {
    let contents = fs::read_to_string("planets.txt").unwrap();

    for line in contents.lines() {
        println!("{}", line);
    }

    let bytes_contents = fs::read("planets.txt").unwrap();
    println!("{:?}", bytes_contents)
}

fn write_file() {
    let mut speech = String::new();
    speech.push_str("We choose to go to the Moon in the decade\n");
    speech.push_str("and do the other things,\n");
    speech.push_str("not because they are easy,\n");
    speech.push_str("but because they are hard.");
    let file_name = "speech.txt";
    let mut file = fs::File::create(file_name).unwrap();

    // Write the content to the file
    file.write_all(speech.as_bytes()).unwrap();
    //Flush the buffer to ensure everything is written to disk
    file.flush().unwrap();
    println!("Content has been written to '{}'", file_name);

    // Sleep for 3 seconds
    thread::sleep(Duration::from_secs(3));
    // Delete the file
    fs::remove_file(file_name).unwrap();

    println!("File '{}' has been deleted", file_name);
}
