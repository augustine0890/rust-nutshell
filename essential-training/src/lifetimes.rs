/*
The borrow checker makes sure the scope of variables to make sure all borrows are valid
- Each input parameter that is a reference is assigned its own lifetime.
- Is there is a &self or &mut self input parameter, its lifetime will be assigned to all output lifetimes.
- If there is exactly one input lifetime, assign it to all output lifetimes.
 */
struct Shuttle<'a> {
    name: &'a str,
}

impl<'a, 'b> Shuttle<'a> {
    fn send_transmission(&'a self, msg: &'b str) -> &'b str {
        println!("Transmitting message ({}): ", self.name);
        msg
    }
}

pub fn run() {
    let vehicle = Shuttle { name: "Endeavour" };
    let sender = vehicle.send_transmission("Greetings from orbit!");
    println!("Sender is {}", sender);

    let propellant;
    {
        let rp1 = String::from("RP-1");
        propellant = &rp1;
        println!("propellant is {}", propellant);
    }

    let result;
    let propellant1 = String::from("RP-1");
    {
        let propellant2 = String::from("LNGING");
        result = best_fuel(&propellant1, &propellant2);
    }

    println!("The result is: {}", result);
}
/*
Use the `Cow` short for clone and write type from the `std::borrow`
 */
fn best_fuel<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        panic!("Function best_fuel should return a value with lifetime 'a");
    }
}
