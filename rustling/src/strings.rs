#![allow(dead_code)]

fn current_favorite_color() -> String {
    "blue".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        let answer = current_favorite_color();
        assert_eq!(answer, String::from("blue"));
    }
}
