use rand;

enum Shot {
    Bullseye,
    Hit { x: f64 },
    Miss,
}

impl Shot {
    fn points(self) -> i32 {
        match self {
            Shot::Bullseye => 5,
            Shot::Miss => 0,
            Shot::Hit { x } => {
                if x < 3.0 {
                    2
                } else {
                    1
                }
            }
        }
    }
}

fn main() {
    // Simulate shooting a bunch of arrows and gathering their coordinates on the target.
    let arrow_coords: Vec<Coord> = get_arrow_coords(5);
    let mut shots: Vec<Shot> = Vec::new();

    // For each coord in arrow_coords
    for coord in arrow_coords {
        coord.print_description();
        let shot = match coord.distance_from_center() {
            s if s < 1.0 => Shot::Bullseye,
            s if s < 5.0 => Shot::Hit { x: s },
            _ => Shot::Miss,
        };
        shots.push(shot);
    }

    // Finally, loop through each shot in shots and add its points to total
    let mut total = 0;
    for shot in shots {
        total += shot.points()
    }
    println!("Final point total is: {}", total);
}

// A coordinate of where an Arrow hit
#[derive(Debug)]
struct Coord {
    x: f64,
    y: f64,
}

impl Coord {
    fn distance_from_center(&self) -> f64 {
        (self.x.powf(2.0) + self.y.powf(2.0)).sqrt()
    }

    fn print_description(&self) {
        println!(
            "coord is {:.1} away, at ({:.1}, {:.1})",
            self.distance_from_center(),
            self.x,
            self.y,
        );
    }
}

// Generate some random coordinates
fn get_arrow_coords(num: u32) -> Vec<Coord> {
    let mut coords: Vec<Coord> = Vec::new();
    for _ in 0..num {
        let coord = Coord {
            x: (rand::random::<f64>() - 0.5) * 12.0,
            y: (rand::random::<f64>() - 0.5) * 12.0,
        };
        coords.push(coord);
    }
    coords
}
