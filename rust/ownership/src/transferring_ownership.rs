fn main() {
    let rocket_fuel = String::from("RP-1");

    // A COPY of rocket_fuel is passed into the function since it lives on the stack
    // We can see this if we made the propellant parameter in the function mutable
    // and then incremented it inside the function, it will not affect anything outside
    // the function

    // If we clone rocket fuel then the value will not be dropped after the other function is done executing
    // process_fuel(rocket_fuel.clone());

    // We can also shadow the first variable
    let rocket_fuel = process_fuel(rocket_fuel);
    println!("rocket_fuel is {}", rocket_fuel);
}

// When we pass rocket_fuel into this we are transferring ownership from the rocket_fuel variable
// to the propellant variable, this means when the propellant variable goes out of scope the data
// inside the variable will be dropped
fn process_fuel(propellant: String) -> String {
    println!("processing propellant {}...", propellant);
    let new_fuel = String::from("LNG");
    new_fuel
}
