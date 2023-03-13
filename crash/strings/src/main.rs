// Silence some warnings so they don't distract from the exercise.
#![allow(dead_code, unused_mut, unused_variables)]

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
