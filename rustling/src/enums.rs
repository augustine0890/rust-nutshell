#![allow(dead_code)]
#[derive(Debug)]
enum Message {
    Move { x: i32, y: i32 },
    Echo(String),
    ChangeColor(u8, u8, u8),
    Quit,
}

impl Message {
    fn call(&self) {
        println!("{:?}", &self);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn message_test() {
        let messages = [
            Message::Move { x: 10, y: 30 },
            Message::Echo(String::from("hello world")),
            Message::ChangeColor(200, 255, 255),
            Message::Quit,
        ];
        for message in messages {
            message.call();
        }
    }
}
