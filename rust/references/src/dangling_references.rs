fn main() {
    let rocket_fuel = produce_fuel();
    println!("rocket_fuel is {}", rocket_fuel);
}

// This function is created a dangling reference because the string gets assigned to new_fuel as
// it's owner, but new_fuel goes out of scope at the end of the function, so the string data will
// be dropped form memory, so when rocket_fuel tries to access the string, the string will no
// longer be there, to fix this you can simply remove the &, so &String -> String and &new_fuel -> new_fuel,
// this makes it so that the data is still available for the rocket_fuel variable to use.
fn produce_fuel() -> String {
    let new_fuel = String::from("RP-1");
    new_fuel
}
