#![allow(dead_code)]

fn run() {
    let mut shopping_list: Vec<&str> = Vec::new();
    shopping_list.push("milk");
    println!("{:?}", shopping_list);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shopping_list() {
        run();
    }
}
