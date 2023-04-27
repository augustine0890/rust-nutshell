#![allow(dead_code)]

fn current_favorite_color() -> String {
    "blue".to_string()
}

fn is_a_color_word(attempt: &str) -> bool {
    attempt == "blue" || attempt == "green" || attempt == "red"
}

fn trim_me(input: &str) -> String {
    // TODO: Remove whitespace from both ends of a string!
    input.trim().to_string()
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There's multiple ways to do this!
    format!("{} world!", input)
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons"!
    input.replace("cars", "balloons")
}

fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        string_slice("blue");
        string("red".to_string());
        string(String::from("hi"));
        string("rust is fun!".to_owned());
        string_slice("nice weather".into());
        string(format!("Interpolation {}", "Station"));
        string_slice(&String::from("abc")[0..1]);
        string_slice("  hello there ".trim());
        string("Happy Monday!".to_string().replace("Mon", "Tues"));
        string("mY sHiFt KeY iS sTiCkY".to_lowercase());

        let word = String::from("green");
        if is_a_color_word(&word) {
            println!("That is a color word I know!");
        } else {
            println!("That is not a color word I know!");
        }

        let answer = current_favorite_color();
        assert_eq!(answer, String::from("blue"));
    }

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(
            replace_me("I think cars are cool"),
            "I think balloons are cool"
        );
        assert_eq!(
            replace_me("I love to look at cars"),
            "I love to look at balloons"
        );
    }
}
