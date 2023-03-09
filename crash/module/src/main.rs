mod animal;
use animal::sound::{cat, dog, fox};
use module::{FIRST, SECOND, THIRD};

fn main() {
    println!("Listening to animail {}", FIRST);
    dog();

    println!("Listening to animail {}", SECOND);
    cat();

    println!("Listening to animail {}", THIRD);
    fox();
}
