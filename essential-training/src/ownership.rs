#[allow(dead_code)]

/*
- The String data type stores the sequence of characters that make up the String on the heap and it
stores a structure containing a pointer to the heap data, the String's length, and its capacity on the stack.
- Dangling reference: a reference to data that no longer exists. It occur when a function returns a reference to a value that
is owned by the variable whose scope is limited to that function.
- A slice only stores a pointer to the heap data, along with length information. The slice doesn't need to keep track of
capacity because the slice will never own anything on the heap.
 */
pub fn run() {
    let mut message = String::from("Greetings from Earth!");
    let first_word = get_first_word(&message);
    println!("This first word is {}", first_word);

    let last_word = get_last_word(&message);
    println!("This last word is {}", last_word);

    let planets = [1, 2, 3, 4, 5, 6, 7, 8];
    let inner_planets = &planets[..5];
    println!("{:?} in inner_planets", inner_planets);

    get_message(&message);
    println!("msg: {}", message);

    add_message(&mut message);
    println!("msg: {}", message);
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

fn get_last_word(msg: &str) -> &str {
    let mut cut_off = 0;
    for (index, ch) in msg.chars().rev().enumerate() {
        if ch == ' ' {
            cut_off = msg.len() - index;
            return &msg[cut_off..];
        }
    }

    &msg[cut_off..]
}

fn get_message(msg: &String) {
    println!("msg: {}", msg);
}

fn add_message(msg: &mut String) {
    msg.push_str(" How's it going")
}
