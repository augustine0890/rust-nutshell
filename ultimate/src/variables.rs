pub fn run() {
    println!("Hello, entire world!");

    let name: &str = "Augustine";
    let age: u8 = 34;
    println!("My name is {} and I am {} years old", name, age);
    let apples: i32 = 10 + 5;
    println!("I have {} apples", apples);
    let answer: i32 = (6 / 2 + 4) * 3;
    println!("The answer is: {}", answer);
    let x: i32 = 5 + 3;
    let x: i32 = x * 2;
    let x: i32 = x - 6;
    let x: i32 = x / 2;
    println!("The answer is: {}", x);
}
