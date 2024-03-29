#![allow(dead_code)]

use std::ops::{Add, Mul, MulAssign};

#[derive(Debug)]
struct Animal {
    legs: i32,
    sound: String,
    size: f32,
}

impl Animal {
    fn new(legs: i32, sound: String, size: f32) -> Self {
        Animal { legs, sound, size }
    }
}

#[derive(Debug)]
struct Product {
    pub name: String,
    pub price: f64,
    stock: f64,
}

impl Product {
    fn new(name: String, price: f64, stock: f64) -> Self {
        Product { name, price, stock }
    }

    fn total_inventory(&self) -> f64 {
        self.price * self.stock
    }
}

#[derive(Debug)]
struct Rectangle<T, U> {
    width: T,
    height: U,
}

impl<T, U> Rectangle<T, U> {
    fn new(width: T, height: U) -> Self {
        Rectangle { width, height }
    }
}

impl<T, U, V> Rectangle<T, U>
where
    T: Mul<U, Output = V> + Copy + Clone,
    U: Mul<V, Output = V> + Copy + Clone,
{
    fn get_area(&self) -> V {
        self.width * self.height
    }
}

impl<T, U> Rectangle<T, U>
where
    T: MulAssign<T>,
    U: MulAssign<U>,
{
    fn scale(&mut self, scalar_width: T, scalar_height: U) {
        self.width *= scalar_width;
        self.height *= scalar_height;
    }
}

fn get_bigger<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

/*
Box<T>: a type of smart pointer that allows you to store data on the heap
- The Box data type consists of pointer on the stack that points to a chunk of memory on the heap that has been allocated
for the data. When the box goes out of scope, it will automatically drop and deallocate the memory it was using on the heap.
 */
fn sum_boxes<T>(a: Box<T>, b: Box<T>) -> Box<T::Output>
where
    T: Add<Output = T>,
{
    Box::new(*a + *b)
}

pub fn run() {
    println!("{:#?}", Animal::new(4, "woof".to_string(), 0.32));
    let shoes = Product::new("My shoes".to_string(), 100.23, 12.0);
    println!("You can't access a private field: {}", shoes.price);
    println!("The product's name is: {}", shoes.name);
    println!(
        "Total inventory for {} is {}",
        shoes.name,
        shoes.total_inventory()
    );
}

#[test]
fn test_sum_boxes() {
    let one = Box::new(1);
    let two = Box::new(2);
    assert_eq!(*sum_boxes(one, two), 3);

    let pi = Box::new(3.14159);
    let e = Box::new(2.71828);
    assert_eq!(*sum_boxes(pi, e), 5.85987);
}

#[test]
fn test_rectangle() {
    let mut rect = Rectangle::new(1.2, 3.4);
    assert_eq!(rect.get_area(), 4.08);
    rect.scale(0.5, 0.5);
    assert_eq!(rect.get_area(), 1.02);
}

#[test]
fn test_get_bigger() {
    assert_eq!(get_bigger(2, 3), 3);
    assert_eq!(get_bigger(4.0, 2.0), 4.0);
    assert_eq!(get_bigger('A', 'C'), 'C');
}
