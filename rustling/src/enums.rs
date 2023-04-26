#![allow(dead_code)]
#[derive(Debug)]
enum Message {
    Quit,
    Echo,
    Move,
    ChangeColor,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn message_test() {
        println!("{:?}", Message::Quit);
        println!("{:?}", Message::Echo);
        println!("{:?}", Message::Move);
        println!("{:?}", Message::ChangeColor);
    }
}
