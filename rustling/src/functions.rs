#![allow(dead_code)]

fn call_me(num: u8) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
fn function1() {
    call_me(3);
}

fn is_even(num: i32) -> bool {
    num % 2 == 0
}

fn sale_price(price: i32) -> i32 {
    if is_even(price) {
        price - 10
    } else {
        price - 3
    }
}

fn function2() {
    let original_price = 51;
    println!("Your sale price is {}", sale_price(original_price));
}

fn square(num: i32) -> i32 {
    num * num
}

fn function3() {
    let answer = square(3);
    println!("The square of 3 is {}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        function1();
        function2();
        function3();
    }
}
