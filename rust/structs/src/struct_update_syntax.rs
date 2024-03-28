#[derive(Debug)]
struct Shuttle {
    name: String,
    crew_size: u8,
    propellant: f64,
}

fn main() {
    let mut vehicle = Shuttle {
        name: String::from("Endeavour"),
        crew_size: 7,
        propellant: 835958.0,
    };

    vehicle.name = String::from("Atlantis");

    // Let's say we want vehicle 2 to be initialized with the same values as the first vehicle, for this we can use the struct update syntax,
    // and this will allow us to copy values from an existing instance, we can do it using ..vehicle, and this tells Rust that any fiels that
    // are not explicitly set should have the same value as the first vehicle. An important thing to note is that if we make changes to the
    // first vehicle then they will not affect the second vehicle

    // Now if we also take out the name property here and just do ..vehicle then it will give a compiler error because the String type can
    // only have one owner, but if we have it use values from the first shuttle then it takes ownership of it, so it will cause an error when
    // we try to print out the value of the first vehicle since vehicle 2 now owns the name property

    // One way to solve this is to clone the first vehicle into the value of the second vehicle with ..vehicle.clone(), but to do this we also
    // have to add #[derive(Clone)] at the top because struct doesn't natively have the ability to clone like this
    let vehicle2 = Shuttle {
        name: String::from("Discovery"),
        ..vehicle
    };

    // An important thing to note is that if we make changes to the first vehicle then they will not affect the second vehicle
    vehicle.crew_size = 6;

    println!("vehicle is {:?}", vehicle);
    println!("vehicle is {:?}", vehicle2);
}
