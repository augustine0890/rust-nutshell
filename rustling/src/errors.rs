#![allow(dead_code)]
use std::error::Error;
use std::num::ParseIntError;

pub fn generate_nametag_text(name: String) -> Result<String, String> {
    if name.is_empty() {
        Err("`name` was empty; it must be nonempty.".to_string())
    } else {
        Ok(format!("Hi! My name is {}", name))
    }
}

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    // let qty = match item_quantity.parse::<i32>() {
    // Ok(q) => q,
    // Err(err) => return Err(err),
    // };
    // let qty = item_quantity.parse::<i32>()?;

    // Ok(qty * cost_per_item + processing_fee)
    item_quantity
        .parse::<i32>()
        .map(|qty| qty * cost_per_item + processing_fee)
}

fn spend_total_cost() -> Result<(), Box<dyn Error>> {
    let mut tokens = 100;
    let pretend_user_input = "8";

    let cost = total_cost(pretend_user_input)?;

    if cost > tokens {
        println!("You can't afford that many!");
    } else {
        tokens -= cost;
        println!("You now have {} tokens.", tokens);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_spend_total_cost() {
        let result = spend_total_cost();

        assert!(result.is_ok());
    }

    #[test]
    fn item_quantity_is_a_valid_number() {
        assert_eq!(total_cost("34"), Ok(171));
    }

    #[test]
    fn item_quantity_is_an_invalid_number() {
        assert_eq!(
            total_cost("beep boop").unwrap_err().to_string(),
            "invalid digit found in string"
        );
    }

    #[test]
    fn generates_nametag_text_for_a_nonempty_name() {
        assert_eq!(
            generate_nametag_text("Beyoncé".into()),
            Ok("Hi! My name is Beyoncé".into())
        );
    }

    #[test]
    fn explains_why_generating_nametag_text_fails() {
        assert_eq!(
            generate_nametag_text("".into()),
            // Don't change this line
            Err("`name` was empty; it must be nonempty.".into())
        );
    }
}
