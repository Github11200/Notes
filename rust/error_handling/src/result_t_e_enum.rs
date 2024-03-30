use std::fs;

fn main() {
    // There may be errors that do not cause the program to fail and can be corrected. To facilitate handling
    // these types of recoverable errors the Rust library has a Result<T, E> enum with Ok(T) and Err(E) that
    // can either succed or fail. This is also included in the prelude

    // If we run this code it will print out Err(Os {...}) with the Err inside the Result enum, but if we change
    // the file path to an actual file it will return Ok(...) with the file contents. To extract the value we
    // can use the .unwrap() method but it shouldn't be used because if it is an error then the program will
    // panic and exit. Instead of using .unwrap() we can use .expect() instead and pass in what we want to display
    // You also shouldn't use this .expect() method generally because there are better ways of handling this enum
    let contents = fs::read_to_string("the_ultimate_question.txt");
    println!("contents is: {:?}", contents);
}
