#![allow(dead_code)]

fn run() {
    println!("This is intro 2");
    println!("Hello, {}!", "World");
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        run();
    }
}
