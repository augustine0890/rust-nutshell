#![allow(dead_code)]

fn run() {
    let x = 5;
    println!("x has the value of: {}", x);
}

fn variable2() {
    let x = 10;
    if x == 10 {
        println!("x is ten!");
    } else {
        println!("x is not ten!");
    }
}

fn variable3() {
    let x: i32 = 8;
    println!("Number {}", x);
}

fn variable4() {
    let mut x = 3;
    println!("Number {}", x);
    x = 5;
    println!("Number {}", x);
}

fn variable5() {
    let number = "T-H-R-E-E";
    println!("Spell a Number : {}", number);
    let number = 3;
    println!("Number plus two is: {}", number + 2);
}

const NUMBER: i32 = 3;
fn variable6() {
    println!("Number {}", NUMBER);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        run();
        variable2();
        variable3();
        variable4();
        variable5();
        variable6();
    }
}
