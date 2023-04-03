use std::any;
use std::fmt;

/*
- Deriving a trait uses a generic, default implement of that trait.
 */
#[derive(PartialEq, PartialOrd)]
struct Satellite {
    name: String,
    velocity: f64, // miles per second
}

struct SpaceStation {
    name: String,
    crew_size: u8,
    altitude: u32, // miles
}

/*
It is contract that specifies certain capabilities that a type must have. Allows to create abstractions and generics.
 */
trait Description {
    fn describe(&self) -> String {
        String::from("An object flying through space!")
    }
}

impl Description for Satellite {
    fn describe(&self) -> String {
        format!(
            "The {} flying at {} miles per second!",
            self.name, self.velocity
        )
    }
}

impl Description for SpaceStation {
    fn describe(&self) -> String {
        format!(
            "The {} flying {} miles high with {} crew members aboard!",
            self.name, self.altitude, self.crew_size
        )
    }
}

impl fmt::Display for Satellite {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmt,
            "{} flying at {} miles per hour",
            self.name, self.velocity
        )
    }
}

/*
Trait bound: allows to write functions that accept many different data types while guaranteeing
that the types it accepts will have the necessary behaviors for your function to use them
 */
fn print_type<T: fmt::Debug>(item: T) {
    println!("{:?} is {}", item, any::type_name::<T>());
}
/*
It makes a function signature more readable by grouping together multiple trait bounds */
// fn compare_and_print<T: fmt::Display + PartialEq + From<U>, U: fmt::Display + PartialEq + Copy>(a: T, b: U) {
fn compare_and_print<T, U>(a: T, b: U)
where
    T: fmt::Display + PartialEq + From<U>,
    U: fmt::Display + PartialEq + Copy,
{
    if a == T::from(b) {
        println!("{} is equal to {}", a, b);
    } else {
        println!("{} is not equal to {}", a, b)
    }
}

pub fn run() {
    compare_and_print(1.0, 1);
    compare_and_print(1.1, 1);

    print_type(2);
    print_type(2.5);
    print_type("The item");
    print_type([12, 15]);

    let hubble = Satellite {
        name: String::from("Hubble Telescope"),
        velocity: 4.72,
    };
    let gps = Satellite {
        name: String::from("GPS"),
        velocity: 2.42,
    };
    println!("hubble is {}", hubble.describe());
    println!("hubble is {}", hubble);
    println!("hubble == gps is {}", hubble == gps);
    println!("hubble > gps is {}", hubble > gps);

    let iss = SpaceStation {
        name: String::from("International Space Station"),
        crew_size: 6,
        altitude: 254,
    };
    println!("iss is {}", iss.describe());
}
