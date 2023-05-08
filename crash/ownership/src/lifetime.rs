#![allow(dead_code)]
/*
Every reference in Rust has a lifetime, which is the scope for which that reference is valid.
The lifetime is to prevent dangling references
Lifetime annotations don't change how long any of the references live. Rather, the describe the relationships of
the lifetime of multiple references to each other without affecting the lifetime
 */

#[derive(Debug)]
struct Person<'a> {
    name: String,
    age: u8,
    favorite_fruit: &'a str,
}

fn get_oldest<'a>(person1: &'a Person<'a>, person2: &'a Person<'a>) -> &'a Person<'a> {
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
        let fruits = vec!["Apples", "Clementines", "Grapefruit", "Kiwis"];
        let p1 = Person {
            name: "John".to_string(),
            age: 20,
            favorite_fruit: fruits[0],
        };
        let p2 = Person {
            name: "Jane".to_string(),
            age: 30,
            favorite_fruit: fruits[1],
        };

        {
            let p3 = get_oldest(&p1, &p2);
            println!("p_oldest: {:?}", p3);
        }
        println!("p1: {:?}", p1);
        println!("p2: {:?}", p2);
    }
}
