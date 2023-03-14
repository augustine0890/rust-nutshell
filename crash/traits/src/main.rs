#![allow(dead_code)]

// Items which implement the `Colorful` trait have a `color` method which will return a string
// describing that item's color.
trait Colorful {
    fn color(&self) -> String;
}

struct Hat {
    size: i8,
}

impl Hat {
    fn new(size: i8) -> Self {
        Self { size: size }
    }
}

impl Colorful for Hat {
    fn color(&self) -> String {
        if self.size >= 0 && self.size < 6 {
            "Red".to_string()
        } else if self.size >= 6 && self.size < 8 {
            "Green".to_string()
        } else {
            "Blue".to_string()
        }
    }
}

impl Colorful for i32 {
    fn color(&self) -> String {
        if is_even(&self) {
            "Orange".to_string()
        } else {
            "Purple".to_string()
        }
    }
}

fn main() {
    let small_hat = Hat { size: 2 };
    let medium_hat = Hat { size: 7 };
    let large_hat = Hat { size: 100 };
    println!("The small hat is {}", small_hat.color());
    println!("The medium hat is {}", medium_hat.color());
    println!("The large hat is {}", large_hat.color());

    for i in 0..=3 {
        println!("{} is {}", i, i.color());
    }

    fortune(small_hat);
    fortune(2);

    let mut carrot = Carrot {
        percent_left: 100.0,
    };
    carrot.bite();
    println!("I take bite: {:?}", carrot);

    let mut grapes = Grapes { amount_left: 100 };
    grapes.bite();
    println!("Eat a grape: {:?}", grapes);

    // function that:
    // - takes a mutable reference to any type that implements Bite
    // - calls `.bite()` several times
    // Hint: Define the generic type between the function name and open paren:
    //       fn function_name<T: Bite>(...)
    //
    bunny_nibbles(&mut carrot);
    println!("Bunny nibbles for awhile: {:?}", carrot);
}

fn is_even(num: &i32) -> bool {
    if num % 2 == 0 {
        true
    } else {
        false
    }
}

fn fortune<T: Colorful>(fortunes: T) {
    println!("The color I see in your future is: {:?}", fortunes.color())
}

trait Bite {
    fn bite(self: &mut Self);
}

#[derive(Debug)] // This enables using the debugging format string "{:?}"
struct Carrot {
    percent_left: f32,
}

impl Bite for Carrot {
    fn bite(self: &mut Self) {
        // Eat 20% of the remaining carrot. It may take awhile to eat it all...
        self.percent_left *= 0.8;
    }
}

#[derive(Debug)]
struct Grapes {
    amount_left: i32,
}

impl Bite for Grapes {
    fn bite(self: &mut Self) {
        self.amount_left -= 1;
    }
}

fn bunny_nibbles<T: Bite>(item: &mut T) {
    item.bite();
    item.bite();
}
