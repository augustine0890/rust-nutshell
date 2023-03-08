// Silence some warnings so they don't distract from the exercise.
#![allow(unused_variables)]

fn main() {
    let number: f64 = 3.989;

    let number = convert_to_integer(number);
    inspect_integer(number);

    let answer: i32 = 42;
    println!("The answer: {}", answer);

    let sum = add(number, answer); // call your `add` function and pass it `number` and `answer` as arguments.
    println!("{} + {} = {}", number, answer, sum);

    let countdown: i32; // declares countdown, but doesn't initialize it
    if answer < 100 {
        countdown = 10;
    } else {
        println!("The answer is clearly wrong.");
        // set countdown to some value here
        countdown = 0;
    }
    println!("The countdown begins at {}", countdown);

    let width: i32 = 4;
    let height: i32 = 7;
    let depth: i32 = 10;

    let area = area_of(width, height);
    println!("Area is {}", area);

    println!("Volumne is {}", volume(width, height, depth));
}

fn inspect_integer(x: i32) {
    println!("The integer is {}", x);
}

fn convert_to_integer(num: f64) -> i32 {
    num.round() as i32
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn area_of(x: i32, y: i32) -> i32 {
    x * y
}

fn volume(x: i32, y: i32, z: i32) -> i32 {
    x * y * z
}
