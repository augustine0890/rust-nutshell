#![allow(dead_code)]

pub fn is_even(num: i32) -> bool {
    num % 2 == 0
}

struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    pub fn new(width: i32, height: i32) -> Rectangle {
        if width <= 0 || height <= 0 {
            panic!("Rectangle width and height cannot be negative!")
        }

        Rectangle { width, height }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_width_and_height() {
        // This test should check if the rectangle is the size that we pass into its constructor
        let rect = Rectangle::new(10, 20);
        assert_eq!(rect.width, 10); // check width
        assert_eq!(rect.height, 20); // check height
    }

    #[test]
    #[should_panic]
    fn negative_width() {
        // This test should check if program panics when we try to create rectangle with negative width
        let _rect = Rectangle::new(-10, 10);
    }

    #[test]
    #[should_panic]
    fn negative_height() {
        // This test should check if program panics when we try to create rectangle with negative height
        let _rect = Rectangle::new(10, -10);
    }
    #[test]
    fn is_true_when_even() {
        assert!(is_even(2));
    }

    #[test]
    fn is_false_when_odd() {
        assert!(!is_even(3));
    }

    #[test]
    fn you_can_assert() {
        assert!(true);
    }

    #[test]
    fn you_can_assert_eq() {
        assert_eq!(true, true);
        assert_eq!(false, false);
        assert_eq!(1, 1);
    }
}
