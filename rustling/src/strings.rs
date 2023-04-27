#![allow(dead_code)]

fn current_favorite_color() -> String {
    "blue".to_string()
}

fn is_a_color_word(attempt: &str) -> bool {
    attempt == "blue" || attempt == "green" || attempt == "red"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        let word = String::from("green");
        if is_a_color_word(&word) {
            println!("That is a color word I know!");
        } else {
            println!("That is not a color word I know!");
        }

        let answer = current_favorite_color();
        assert_eq!(answer, String::from("blue"));
    }
}
