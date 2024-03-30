fn main() {
    // A vector holds a collection of elements with the same data type and stores them sequentially, and the difference
    // between vectors and arrays is that arrays need to have a fixed size which must be known at compile time since
    // they are stored on the stack, but with vectors items can be dynamically added and removed and it is stored in the
    // heap and we need to handle ownership and borrowing

    let mut astronauts: Vec<String> = Vec::new();
    astronauts.push(String::from("Shepard")); // Alan Shepard
    astronauts.push(String::from("Grissom")); // Gus Grissom
    astronauts.push(String::from("Glenn")); // John Glenn
    println!("astronauts is {:?}", astronauts);

    let last = astronauts.pop();
    println!("last is {:?}", last);

    // Since the vector owns the value it's storing we need to reference it using the borrow operator
    // let third = &astronauts[2];
    // The get method is safer because it won't cause the program to panic and return an Option enum
    let third = astronauts.get(2);
    println!("third is {:?}", third);

    // You can prepopulate a vector as shown below
    let countdown = vec![5, 4, 3, 2, 1];
}
