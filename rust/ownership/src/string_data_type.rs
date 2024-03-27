fn main() {
    // The string data type gets stored in the heap.

    // When we have something like "Hello" it is called a string literal because it is
    // literally hard coded into the code. String literals are immutable though, and
    // it has to be known before compilation.

    // In order to dynamically add string data we can use a string type

    // String types have the data dynamically allocated onto the heap so the string
    // can be mutable, and it can be dynamically generated.

    // The :: is a path operator so we can access the from function associated with the String type
    // The message variable will hold a pointer to the first letter of the string, the length of
    // the string, and the capacity which is how much room is allocated to it (you could have
    // extra space allocated which lets it grow if it needs to)
    let mut message = String::from("Earth");
    println!("Message is {}", message);
    message.push_str(" is home.");
    println!("Message is {}", message);
}
