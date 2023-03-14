// Silence some warnings so they don't distract from the exercise.
#![allow(dead_code, unused_mut)]

use structs::Polygon;

fn main() {
    let mut polygon = Polygon::new("George".to_string());
    println!(
        "I see a {}-sided polygon named {}!",
        polygon.sides(),
        polygon.name()
    );

    println!(
        "The polygon named {} is a {}",
        polygon.name(),
        polygon.shape()
    );

    for _ in 0..3 {
        polygon.increment_sides();
        println!(
            "The polygon now has {} sides and is the shape of a {}",
            polygon.sides(),
            polygon.shape()
        );
    }
}
