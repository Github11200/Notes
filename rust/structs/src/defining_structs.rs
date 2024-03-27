// It is common to capitalize the name of a struct
// This definition lets Rust know what goes into this Shuttle but we need a decleration
#[derive(Debug)] // We have to add this to print the struct and it is a trait which we will cover later
struct Shuttle {
    name: String,
    crew_size: u8,
    propellant: f64,
}

fn main() {
    // Tuples are useful when we need to group multiple items of mixed data types, but elements are ordered
    // so it can be hard to keep track of the items. This is where a struct comes it

    // A struct lets you group multiple items of mixed data types and it lets you name the elements so you
    // don't have to worry about the order that the elements are stored in

    // We can declare the struct here
    let mut vehicle = Shuttle {
        name: String::from("Endeavour"),
        crew_size: 7,
        propellant: 835958.0,
    };

    // We can access properties with a .
    println!("name is {}", vehicle.name);

    // We can also change values because it is a mutable struct
    vehicle.name = String::from("Atlantis");
    println!("vehicle is {:?}", vehicle);

    // Struct data will be kept on the stack unless you explicitly say to store it on the heap
}
