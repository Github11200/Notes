fn main() {
    // If we want to change the value of a borrowed variable then we have to explicitly tell Rust that
    // Once you create a mutable reference you cannot create other references in that scope, this
    // prevents a data race where multiple references can access the same data, this is concern when
    // there are multiple threads happening concurently

    // We have to make rocket_fuel mut, do &mut rocket_fuel, and &mut String in the function parameter
    let mut rocket_fuel = String::from("RP-1");
    let length = process_fuel(&mut rocket_fuel);
    println!("rocket_fuel is {} and length is {}", rocket_fuel, length);
}

fn process_fuel(propellant: &mut String) -> usize {
    println!("Processing propellant {}...", propellant);
    propellant.push_str(" is highly flammable!");
    let length = propellant.len();
    length
}

// In rust you can have 1 mutable reference:
//                      let ref1 = &mut var;
// Or you can have more than 1 immutable references:
//                      let ref1 = &var;
//                      let ref2 = &var;
// But you can't have both in the same scope (so the code below will not work):
//                      let ref1 = &mut var;
//                      let ref2 = &var;
