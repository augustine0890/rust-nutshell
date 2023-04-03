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

pub fn run() {
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
}
