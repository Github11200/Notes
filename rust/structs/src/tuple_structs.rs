// An example is if we have a tuple that represents a color, (52, 152, 219), so we can create
// a tuple struct as shown below

// Now Color and Point have the same data types, but since they are tuple structs we can differentiate them using their name
struct Color(u8, u8, u8); // RGB
struct Point(u8, u8, u8); // XYZ

fn get_y(p: Point) -> u8 {
    p.1
}

fn main() {
    // Tuple structs store a collection of mixed data without named fields, but they can be distinguishable as a unique data type, and this
    // can be useful whe you want to give a whole tuple a name to differentiate it from other tuples

    let red = Color(255, 0, 0);

    // Now since these fields are not named we can access them like normal with their indexes
    println!("First value is {}", red.0);

    let coord = Point(4, 5, 6);
    let y = get_y(coord);
    println!("y is {}", y);
}
