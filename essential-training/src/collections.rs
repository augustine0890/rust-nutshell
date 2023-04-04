use std::collections::HashMap;

pub fn run() {
    let mut astronauts: Vec<String> = Vec::new();
    astronauts.push(String::from("Shepard"));
    astronauts.push(String::from("Grissom"));
    astronauts.push(String::from("Glenn"));
    println!("Astronauts is {:?}", astronauts);

    // let third = &astronauts[2];
    let third = match astronauts.get(2) {
        Some(value) => value,
        None => "No third Astronaut",
    };
    println!("The third Astronaut is {}", third);

    let last = astronauts.pop().unwrap();
    println!("The last Astronaut is {}", last);

    println!("Astronauts is {:?}", astronauts);

    let countdown = vec![5, 4, 3, 2, 1];
    println!("{:?}", countdown);

    let mut missions_flown = HashMap::new();
    missions_flown.insert("Hadfield", 3);
    missions_flown.insert("Hurley", 3);
    missions_flown.insert("Barron", 2);
    println!("Missions Flown: {:?}", missions_flown);
    missions_flown.entry("Augustine").or_insert(1);
    println!("Missions Flown: {:?}", missions_flown);

    let barron_missions = match missions_flown.get("Barron") {
        Some(v) => v,
        None => &0,
    };
    println!("Barron's missions: {}", barron_missions);

    let kayla = missions_flown.entry("Kayla").or_insert(0);
    *kayla += 1;
    let kayla_missions = match missions_flown.get("Kayla") {
        Some(v) => v,
        None => &0,
    };
    println!("Kayla's missions: {}", kayla_missions);
}
