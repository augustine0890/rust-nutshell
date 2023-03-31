#![allow(dead_code)]

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
struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn new(width: f64, height: f64) -> Self {
        Rectangle { width, height }
    }

    fn get_area(&self) -> f64 {
        self.width * self.height
    }

    fn scale(&mut self, scalar: f64) {
        self.width *= scalar;
        self.height *= scalar;
    }
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
fn test_rectangle() {
    let mut rect = Rectangle::new(1.2, 3.4);
    assert_eq!(rect.get_area(), 4.08);
    rect.scale(0.5);
    assert_eq!(rect.get_area(), 1.02);
}
