#![allow(dead_code)]

fn run() {
    let mut shopping_list: Vec<&str> = Vec::new();
    shopping_list.push("milk");
    println!("{:?}", shopping_list);
}

struct Wrapper<T> {
    value: T,
}

impl<T> Wrapper<T> {
    pub fn new(value: T) -> Self {
        Wrapper { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }

    #[test]
    fn test_shopping_list() {
        run();
    }
}
