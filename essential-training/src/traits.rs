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

pub fn run() {
    let hubble = Satellite {
        name: String::from("Hubble Telescope"),
        velocity: 4.72,
    };
    let gps = Satellite {
        name: String::from("GPS"),
        velocity: 2.42,
    };
    println!("hubble is {}", hubble.describe());
    println!("hubble == gps is {}", hubble == gps);
    println!("hubble > gps is {}", hubble > gps);

    let iss = SpaceStation {
        name: String::from("International Space Station"),
        crew_size: 6,
        altitude: 254,
    };
    println!("iss is {}", iss.describe());
}
