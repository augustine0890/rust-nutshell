// mod arguments;
// mod collections;
// mod enums;
// mod error;
// mod lifetimes;
// mod module;
// mod ownership;
// mod structs;
// mod traits;

use std::{collections::HashMap, env, fs};

fn main() {
    let mut value = 0b1111_0101u8;
    println!("The value is: {}", value);
    println!("The binary representation: {:08b}", value);

    value = !value; // NOT
    println!("The value is: {}", value);
    println!("The binary representation: {:08b}", value);

    value = value & 0b1111_0111; // clear bit with AND
    println!("The binary representation: {:08b}", value);
    println!("bit 6 is {}", value & 0b0100_0000); // check bit with AND

    value = value | 0b0100_0000; // set bit with OR
    println!("The binary representation: {:08b}", value);

    value = value ^ 0b0101_0101; // XOR
    println!("The binary representation: {:08b}", value);

    value = value << 4;
    // println!("The value is: {}", value);
    println!("The binary representation: {:08b}", value);

    value = value >> 2;
    println!("The binary representation: {:08b}", value);

    let a = true | false;
    println!("true bitwise OR false is {}", a);
    let b = true || false;
    println!("true logical OR false is {}", b);

    // The | operator operates on integers at the bit level, while || operates on boolean values.
    let x = 0b1010; // This is binary literal for decimal 10
    let y = 0b1100; // This is binary literal for decimal 12
                    // Bitwise OR
    let z = x | y;
    println!("Bitwise OR: {}", z); // Output: 14 (binary 1110)

    // Logical OR
    if x > 5 || y > 15 {
        println!("At least one operand is true");
    } else {
        println!("Both operands are false");
    }

    let numbers: [i32; 5];
    numbers = [0; 5];
    let index = numbers.len() - 1;
    println!("last number is {}", numbers[index]);

    let message = ['a', 'u', 'g', 'u', 's', 't', 'i', 'n', 'e'];
    for (i, c) in message.iter().enumerate() {
        println!("item {} is {}", i, c);
        if *c == 't' {
            break;
        }
    }

    let numbers = [1, 9, -2, 0, 23, 20, -7, 13, 37, 20, 56, -18, 20, 3];
    let mut max: i32 = numbers[0];
    let mut min: i32 = numbers[0];
    let mut sum: f64 = 0.0;

    for num in numbers {
        if max < num {
            max = num;
        } else if min > num {
            min = num;
        }

        sum += num as f64;
    }

    let mean = sum / numbers.len() as f64;

    assert_eq!(max, 56);
    assert_eq!(min, -18);
    assert_eq!(mean, 12.5);
    println!("Tests (max, min, mean) passed!");

    // ownership::run();

    // module::run();

    // arguments::run();

    // structs::run();

    // traits::run();

    // lifetimes::run();

    // enums::run();

    // error::run();

    // collections::run();

    count_words();
}

fn count_words() {
    let parsed_args: Vec<String> = env::args().skip(1).collect();
    if parsed_args.len() < 1 {
        eprintln!("Program requires an argument: <file path>");
        std::process::exit(2);
    }

    let contents = match parsed_args.get(0) {
        Some(name) => match fs::read_to_string(&name) {
            Ok(content) => content,
            Err(_) => {
                eprintln!("Error: Failed to read the file: {}", name);
                std::process::exit(1);
            }
        },
        None => {
            eprintln!("Error: No file name provided");
            std::process::exit(1);
        }
    };

    let mut word_counts = HashMap::new();
    for word in contents.split_whitespace() {
        let cleaned_word = clean_word(word);
        let counter = word_counts.entry(cleaned_word).or_insert(0);
        *counter += 1;
    }

    let mut sorted_word_counts: Vec<(&String, &usize)> = word_counts.iter().collect();
    sorted_word_counts.sort_by(|b, a| a.1.cmp(b.1));

    println!("Top 10 most frequent words:");
    for (word, count) in sorted_word_counts.iter().take(10) {
        println!("{word}: {count}");
    }
}

// Removes non-alphanumeric characters and converts the word to lowercase.
fn clean_word(word: &str) -> String {
    word.trim_matches(|c: char| !c.is_alphabetic())
        .to_lowercase()
}
