fn main() {
    // In ownership resources can only have one owner at one time

    let outer_planet: String;
    let integer_outer_planet: i32;
    {
        let mut inner_planet = String::from("Mercury");
        println!("inner_planet is {}", inner_planet);

        // We can assign inner planet to outer planet to use it outside of this scope
        // Since you can only have one owner Rust invalidates inner_planet because we've
        // assigned the value to outer_planet. This is called a move
        outer_planet = inner_planet;

        // If we try printing out it will throw and error because the value has been moved
        // println!("inner_planet is {}", inner_planet);

        // If we wanted to do a deep copy where each variable will be an owner of a seperate
        // independent string then we can use the .copy() method
        // outer_planet = inner_planet.clone();

        // If we use integers intead then we don't have to use .clone and it will automatically
        // be two seperate copies, so it will print 2 in this scope and 1 in the outer scope
        // This is because when we define the inner_planet it is being defined in the stack
        let mut integer_inner_planet = 1;
        integer_outer_planet = integer_inner_planet;
        integer_inner_planet += 1;
        println!("integer_inner_planet is {}", integer_inner_planet);
    }
    println!("outer_planet is {}", outer_planet);
    println!("integer_outter_planet is {}", integer_outer_planet);

    // Data types with a known size which are stored on the stack will be copied instead of moved like
    // a string which on the heap.

    // Copying data on the stack happens implicitly while cloning on the heap has to be done explicitly
}
