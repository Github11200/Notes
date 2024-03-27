fn main() {
    // Rust let's use create a new variable with the same name as an existing variablke
    // When we do this the new variable "shadows" the previous variable, so when we access
    // that new variable name we'll see the value of the second variable which is shadowing
    // the first one

    let planet = "Earth";
    {
        // If we shadow it within a different scope then the value won't always be changed
        println!("Planet is {}", planet);
        let planet = 4; // We can shadow the first variable and also change the data type
        println!("Planet is {}", planet);
    }
    println!("Planet is {}", planet);
}
