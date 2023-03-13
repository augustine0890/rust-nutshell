// Silence some warnings so they don't distract from the exercise.
#![allow(dead_code, unused_mut, unused_variables)]

// Q. What's the difference between a string literal and a borrowed string slice?
//
// A. A string literal is what is written in your source code. e.g. "this is a string literal",
// while a borrowed strings slice (&str) is the *type* of the string literal. So:
//
//   let my_name: &str = "Augustine";
//
// The variable my_name is a borrowed string slice, initialized by the string literal "Augustine".

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    for arg in args {
        if arg == "sum" {
            sum();
        } else if arg == "double" {
            double();
        } else {
            count(arg);
        }
    }

    let mut count = 0u32;
    let mut bunnies = 2u32;
    while bunnies < 500 {
        count += 1;
        bunnies *= 2;
    }
    println!(
        "Bunnies doubled {} times before there were more than 500",
        count
    );

    let mut fives: Vec<i32> = vec![];
    let mut numeber: i32 = 5;
    for i in 1..=12 {
        fives.push(i * numeber);
    }
    println!("Here are the first 12 multiples of 5: {:?}", fives);

    let mut total = 0u32;
    let numbers = vec![0, 1, 2, 3, 4, 5];
    for number in numbers {
        if number == 0 {
            total += 7;
        } else if number == 1 || number == 2 {
            total += 30;
        } else {
            total -= 5;
        }
    }
    println!("The total is {}", total);

    // Using unicode escape codes, use println to print out a sparkles emoji (codepoint 2728).
    println!("\u{2728}");

    let mut favorite = String::new();
    let favorite = "\u{1f353}".to_string();
    if favorite != "" {
        println!("My favorite fruit is: {favorite}");
    } else {
        println!("No favorite fruit specified");
    }

    let saying = "Now is\nthe time\nfor all\ngreat men";
    println!("{saying}")
}

fn sum() {
    let mut sum = 0;
    for i in 7..=23 {
        sum += i;
    }
    println!("The sum is {}", sum);
}

fn double() {
    let mut count = 0;
    let mut x = 1;
    while x < 500 {
        x *= 2;
        count += 1;
    }
    println!(
        "You can double x {} times until x is larger than 500",
        count
    );
}

fn count(arg: String) {
    let mut count = 0u32;
    loop {
        count += 1;
        if count == 9 {
            break;
        }
        print!("{} ", arg);
    }

    println!();
}
