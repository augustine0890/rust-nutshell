#![allow(dead_code)]
/*
Every reference in Rust has a lifetime, which is the scope for which that reference is valid.
The lifetime is to prevent dangling references
Lifetime annotations don't change how long any of the references live. Rather, the describe the relationships of
the lifetime of multiple references to each other without affecting the lifetime
 */

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

fn get_oldest<'a>(person1: &'a Person, person2: &'a Person) -> &'a Person {
    if person1.age > person2.age {
        person1
    } else {
        person2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        let p1 = Person {
            name: "John".to_string(),
            age: 20,
        };
        let p2 = Person {
            name: "Jane".to_string(),
            age: 30,
        };

        {
            let p3 = get_oldest(&p1, &p2);
            println!("p_oldest: {:?}", p3);
        }
        println!("p1: {:?}", p1);
        println!("p2: {:?}", p2);
    }
}
