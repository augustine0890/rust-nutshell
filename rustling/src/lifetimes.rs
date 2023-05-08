#![allow(dead_code)]

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct Book<'a> {
    author: &'a str,
    title: &'a str,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_book_struct() {
        let name = String::from("Jill Smith");
        let title = String::from("Fish Flying");
        let book = Book {
            author: &name,
            title: &title,
        };

        println!("{} by {}", book.title, book.author);
    }

    #[test]
    fn test_longest() {
        let string1 = String::from("abcd");
        let string2 = "xyz";

        let result = longest(string1.as_str(), string2);
        println!("The longest string is: {}", result);
    }

    #[test]
    fn test_longest_2() {
        let string1 = String::from("long string is long");
        let result;
        {
            let string2 = String::from("xyz");
            result = longest(string1.as_str(), string2.as_str());
            println!("The longest string is '{}'", result);
        }
    }
}
