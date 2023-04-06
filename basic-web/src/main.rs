mod to_do;
use to_do::structs::done::Done;
use to_do::structs::pending::Pending;

fn main() {
    let done = Done::new("learning");
    println!("{}", done.super_struct.title);
    println!("{}", done.super_struct.status);

    let pending = Pending::new("some jobs");
    println!("{}", pending.super_struct.title);
    println!("{}", pending.super_struct.status);
}
