/*
- The String data type stores the sequence of characters that make up the String on the heap and it
stores a structure containing a pointer to the heap data, the String's length, and its capacity on the stack.
 */
pub fn run() {
    let message = String::from("Greetings from Earth!");
    let first_word = get_first_word(&message);
    println!("This first word is {}", first_word);
}

fn get_first_word(msg: &str) -> &str {
    let bytes = msg.as_bytes();

    for (index, byte) in bytes.iter().enumerate() {
        if *byte == b' ' {
            return &msg[..index];
        }
    }

    &msg
}

