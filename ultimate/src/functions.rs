fn say_apples(apples: i32) {
    println!("I have {} apples", apples);
}

fn double_apples(apples: i32) -> i32 {
    apples * 2
}

fn eat(count: i32, food: &str) {
    println!("You ate {} helpings of {}", count, food);
}

fn greet(name: &str, number: i32) {
    println!("Hello {}, you can have {} fruits", name, number);
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn subtract(x: i32, y: i32) -> i32 {
    x - y
}

#[allow(unused_variables, unused_mut)]
fn quadruple(x: i32) -> i32 {
    (x * 2) * (x * 2)
}

pub fn func_run() {
    let mut apples: i32 = 11;
    say_apples(apples);
    println!("I ate an apple!");
    apples = apples - 1;
    say_apples(apples);
    println!("I ate an apple!");
    apples = apples - 1;
    say_apples(apples);

    println!("Double apple!");
    apples = double_apples(apples);
    say_apples(apples);

    eat(5, "apples");
    eat(8, "bananas");

    greet("Alice", 12);
    greet("Bob", 10);
    greet("Charlie", 8);

    let lemons: i32 = 10;
    println!("Before the block, I have {} lemons", lemons);
    {
        let lemons: i32 = lemons + 2;
        println!("Inside the block, I have {} lemnons", lemons);
    }
    println!("After the block, I have {} lemnons", lemons);

    let a = add(3, 4);
    let b = add(1, 6);
    println!("{}", subtract(a, b));

    println!("{}", quadruple(3));
}
