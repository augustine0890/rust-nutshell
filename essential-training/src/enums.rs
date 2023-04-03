#![allow(dead_code)]

use std::f64::consts;

#[derive(Debug)]
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64, f64),
}

impl Shape {
    fn get_perimeter(&self) -> f64 {
        match self {
            Shape::Circle(r) => r * 2.0 * consts::PI,
            Shape::Rectangle(w, h) => (w + h) * 2.0,
            Shape::Triangle(a, b, c) => a + b + c,
        }
    }
}

enum Location {
    Unknown,
    Anonymous,
    Known(f64, f64),
}

impl Location {
    fn display(&self) {
        match self {
            Location::Unknown => println!("Unknown location"),
            Location::Anonymous => println!("Anonymous location"),
            Location::Known(x, y) => println!("The location: ({}, {})", x, y),
        }
    }
}

/*
enum: defines a data type with multiple possible variants.
 */
pub fn run() {
    let address = Location::Unknown;
    address.display();
    let address = Location::Anonymous;
    address.display();
    let address = Location::Known(28.608295, -80.604177);
    address.display();

    let my_shape = Shape::Circle(2.5);
    println!("My shape: {:?}", my_shape);
    println!("Perimeter is: {}", my_shape.get_perimeter());

    match my_shape {
        Shape::Circle(r) => println!("Circle with radius: {:?}", r),
        Shape::Rectangle(w, h) => println!("Rectangle with height {} and width {}", h, w),
        Shape::Triangle(a, b, c) => println!("Triangle with the sides {} {} {}", a, b, c),
    }

    let my_number = 1u8;
    let result = match my_number {
        0 => "zero",
        1 => "one",
        2 => "two",
        _ => {
            println!("{} did not match", my_number);
            "something else"
        }
    };
    println!("The result is: {}", result);

    let countdown = [5, 4, 3, 2, 1];
    let number = countdown.get(2);
    let number_plus_one = match number {
        Some(n) => *n + 1,
        None => 0,
    };
    println!("The number plus one is: {:?}", number_plus_one);
}
