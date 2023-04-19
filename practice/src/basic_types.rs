#![allow(unused_variables, dead_code)]

pub fn run() {
    let x: i32 = 5;
    let mut y: u32 = 10;
    println!("{}", y);

    y = x as u32;
    assert_eq!(y, 5);
    let z = 10;
    assert_eq!(z, 10);

    let v: u16 = 38_u8 as u16;
    assert_eq!(v, 38);
}

fn integer() {
    let x = 5;
    assert_eq!("i32".to_string(), type_of(&x));

    assert_eq!(i8::MAX, 127);
    assert_eq!(u8::MAX, 255);

    let v1 = 251_u8 + 4;
    let v2 = i8::checked_add(127, 0).unwrap();
    println!("{}, {}", v1, v2);

    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    assert!(v == 1597);
}

fn floating() {
    let x = 1_000.000_1;
    let y: f32 = 0.12;
    let z = 0.1_f64;
    assert_eq!(type_of(&x), "f64".to_string());

    assert!(0.1 + 0.2 == 0.3_f32);
}

fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

fn range() {
    let mut sum = 0;
    for i in -3..3 {
        sum += i
    }

    assert!(sum == -3);

    for c in 'a'..='z' {
        print!("{}", c);
    }
    println!();

    use std::ops::{Range, RangeInclusive};
    assert_eq!((1..5), Range { start: 1, end: 5 });
    assert_eq!((1..=5), RangeInclusive::new(1, 5));
}

fn computations() {
    // Integer addition
    assert!(1u32 + 2 == 3);

    // Integer subtraction
    assert!(1i32 - 2 == -1);
    assert!(1u8 - 1 == 0 as u8);

    assert!(3 * 50 == 150);

    assert!(9.6 / 3.2 == 3.0 as f32); // error ! make it work

    assert!(24 % 5 == 4);
    // Short-circuiting boolean logic
    assert!(true && false == false);
    assert!(true || false == false);
    assert!(!true == false);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integer() {
        integer();
    }

    #[test]
    fn test_floating() {
        floating();
    }

    #[test]
    fn test_range() {
        range();
    }

    #[test]
    fn test_computations() {
        computations();
    }
}
