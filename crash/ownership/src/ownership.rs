#![allow(dead_code)]

/*
- Each value in Rust has an owner
- There can only be one owner at a time
- When the owner goes out of scope, the value will be dropped
- Data types that implement the copy trait ignore the Ownership Rules
- The copy trait is implemented by the primitive data types stored on the stack
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        rule_ownership();

        let x = String::from("41");
        print_string(&x);
        print_string(&x);
    }
}
