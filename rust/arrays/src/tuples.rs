fn main() {
    // Tuples can store elements of either the same OR different type
    let mut stuff: (u8, f32, char) = (10, 3.14, 'x'); // You can specify different datatypes by putting them inside brackets
    stuff.0 += 3; // We can modify values like this
    let first_item = stuff.0; // For accessing tuples you have ot use . but it is also 0 indexed

    println!("first item is {}", first_item);

    let (a, b, c) = stuff; // We can destructure tuples as well
    println!("a is {}", a);
    println!("b is {}", b);
    println!("c is {}", c);
}
