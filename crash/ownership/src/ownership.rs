#![allow(dead_code)]

/*
- Each value in Rust has an owner
- There can only be one owner at a time
- When the owner goes out of scope, the value will be dropped
- Data types that implement the copy trait ignore the Ownership Rules
- The copy trait is implemented by the primitive data types stored on the stack
- Use String when you need to own the string data
- Use &str when you only need to borrow a string
*/

fn rule_ownership() {
    let x: [i32; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
    let y = x;
    println!("{:?}", x);
    println!("{:?}", y);
}

fn print_string(a: &String) {
    println!("This is the value of my string: {}", a);
}

#[derive(Debug)]
struct Person {
    id: u8,
    age: u8,
    name: String,
}

impl Clone for Person {
    fn clone(&self) -> Self {
        println!("Copied from this old value {:?}", self);

        Person {
            id: self.id,
            age: self.age,
            name: self.name.clone(),
        }
    }
}

fn print_length(x: &str) {
    println!("The length of '{}' is {}.", x, x.len());
}

fn danggling_ref() -> String {
    // let s = "hello world".to_string();
    // &s
    "hello world".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reference() {
        danggling_ref();

        let mut x = 45;

        let y = &mut x;
        *y += 1;
        println!("y = {}", y);

        println!("x = {}", x);
    }

    #[test]
    fn test() {
        let mut s = String::from("This is String");
        print_length(&s);

        let y = &mut s;
        y.push_str("!");
        print_length(y);

        let z = &s[..];
        print_length(z);

        let p1 = Person {
            id: 1,
            age: 29,
            name: "augustine".to_string(),
        };

        let mut p2 = p1.clone();
        p2.id = 2;
        p2.age = 32;
        p2.name = "jack".to_string();

        println!("{:?}", p1);
        println!("{:?}", p2);

        let x = String::from("41");
        print_string(&x);
        print_string(&x);

        rule_ownership();
    }
}
