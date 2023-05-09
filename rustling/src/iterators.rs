#![allow(dead_code)]
// Step 1.
// Complete the `capitalize_first` function.
// "hello" -> "Hello"
pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(first) => {
            let capital = first.to_uppercase().to_string();
            format!("{capital}{}", c.as_str())
        }
    }
}

// Step 2.
// Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    words
        .iter()
        .map(|word| capitalize_first(word))
        .collect::<Vec<String>>()
}

// Step 3.
// Apply the `capitalize_first` function again to a slice of string slices.
// Return a single string.
// ["hello", " ", "world"] -> "Hello World"
pub fn capitalize_words_string(words: &[&str]) -> String {
    words
        .iter()
        .map(|word| capitalize_first(word))
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }

    #[test]
    fn test_run() {
        let my_fav_fruits = vec!["banana", "custard apple", "avocado", "peach", "raspberry"];

        let mut my_iterable_fav_fruits = my_fav_fruits.iter();

        assert_eq!(my_iterable_fav_fruits.next(), Some(&"banana"));
        assert_eq!(my_iterable_fav_fruits.next(), Some(&"custard apple")); // TODO: Step 2
        assert_eq!(my_iterable_fav_fruits.next(), Some(&"avocado"));
        assert_eq!(my_iterable_fav_fruits.next(), Some(&"peach")); // TODO: Step 3
        assert_eq!(my_iterable_fav_fruits.next(), Some(&"raspberry"));
        assert_eq!(my_iterable_fav_fruits.next(), None); // TODO: Step 4
    }
}
