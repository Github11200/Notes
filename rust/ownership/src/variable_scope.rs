fn main() {
    // Scope is the region of the program where a variable is valid
    // A variable is valid when it comes into scope and stays valid
    // until it goes out of scope

    // Variable bindings are constrained to live within a block of code
    // These blocks are usually enclosed by curly braces (fucntions,
    // loops, etc)

    if true {
        let planet = "Earth"; // This variable is within the scope of this if statement
        println!("Planet is {}", planet);
    }
}
