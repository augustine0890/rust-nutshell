pub fn run() {
    println!("Hello, entire world!");

    let name: &str = "Augustine";
    let age: u8 = 34;
    println!("My name is {} and I am {} years old", name, age);

    // The semicolon throws away the result of an expression
    // If you end your block with an expression without a semicolon, it will produce that value, otherwise it produces unit
    let apples: i32 = {
        println!("I'm about to figure out how many apples there are");
        let x = 10 + 5;
        println!("Now I know how many apples there are");
        x
    };
    println!("I have {} apples", apples);

    let answer: i32 = (6 / 2 + 4) * 3;
    println!("The answer is: {}", answer);
    let x: i32 = 5 + 3;
    let x: i32 = x * 2;
    let x: i32 = x - 6;
    let x: i32 = x / 2;
    println!("The answer is: {}", x);
}
