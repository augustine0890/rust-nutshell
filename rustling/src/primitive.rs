#![allow(dead_code)]

fn bool() {
    let is_morning = true;
    if is_morning {
        println!("Good morning!");
    }

    let is_evening = !is_morning;
    if is_evening {
        println!("Good evening!");
    }
}

fn char() {
    let my_first_initial = 'C';
    if my_first_initial.is_alphabetic() {
        println!("Alphabetical!");
    } else if my_first_initial.is_numeric() {
        println!("Numerical!");
    } else {
        println!("Neither alphabetical nor numeric!");
    }

    let your_character = '🧧';
    if your_character.is_alphabetic() {
        println!("Alphabetical!");
    } else if your_character.is_numeric() {
        println!("Numerical!");
    } else {
        println!("Neither alphabetic nor numeric!");
    }
}

fn array() {
    let a = [1, 2, 3, 4, 5];
    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
    }

    let nice_slice = &a[0..3];
    assert_eq!([1, 2, 3], nice_slice);
}

fn tuple() {
    let cat = ("Furry McFurson", 3.5);
    let name = cat.0;
    let age = cat.1;
    println!("{} is {} years old.", name, age);

    let numbers = (1, 2, 3);
    let second = numbers.1;
    assert_eq!(2, second, "This is not the 2nd number in the tuple!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        bool();
        char();
        array();
        tuple();
    }
}
