#![allow(dead_code)]

pub fn run() {
    let x: i32 = 5;
    let y: i32 = 10;
    println!("{} is equal to 5 and y is equal to {}", x, y);

    let mut plus = 1;
    plus += 2;
    assert_eq!(plus, 3);
}

fn scope() {
    let x: i32 = 10;
    let y: i32 = 15;
    {
        let y: i32 = 5;
        println!("The value of x is {} and value of y is {}", x, y);
    }
    println!("The value of x is {} and value of y is {}", x, y);

    let s = define_s();
    println!("{}, world", s);
}

fn define_s() -> String {
    "hello".to_string()
}

fn shadow() {
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 12);
    }
    assert_eq!(x, 5);

    let x = 42;
    println!("{}", x);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scope() {
        scope();
    }

    #[test]
    fn test_shadow() {
        shadow();
    }

    #[test]
    fn test_shadow_rebind() {
        let mut x: i32 = 1;
        println!("{}", x);
        x = 7;
        // Shadowing and re-binding
        x += 3;
        println!("{}", x);

        let y = 4;
        println!("{}", y);
        let y = "I can also be bound to text!";
        println!("{}", y);
        let _z = 1;
    }

    #[test]
    fn test_destructure() {
        let (mut x, y) = (1, 2);
        x += 2;

        assert_eq!(x, 3);
        assert_eq!(y, 2);
    }

    #[test]
    fn test_destructure_assignement() {
        let (x, y);
        (x, ..) = (3, 4);
        [.., y] = [1, 2];
        assert_eq!([x, y], [3, 2]);
    }
}
