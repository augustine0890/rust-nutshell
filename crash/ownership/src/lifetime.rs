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

impl<'a> Person<'a> {
    fn print_and_return(&self, announcement: &str) -> &str {
        println!("{}", announcement);
        println!("{}'s favorite fruit is {}", self.name, self.favorite_fruit);
        self.favorite_fruit
    }
}
/*
Lifetime Elision Rules
1. The compiler assigns a lifetime parameter to each parameter that's a reference.
2. If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters.
3. If there are multiple input lifetime parameters, but one of them is &self of &mut self, the lifetime of self is assigned
to all output lifetime parameters. */
fn first_word_in_string(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        let sentences = vec!["Hello world!", "How are you?", "I am fine", "Good"];
        for sentence in sentences.iter() {
            println!(
                "The first word in the [{}] is: {}",
                sentence,
                first_word_in_string(sentence)
            );
        }

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
        p2.print_and_return("Everyone listen up!");
    }
}
