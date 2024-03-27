fn main() {
    let message = ["h", "e", "l", "l", "o"];

    // Before running the loop Rust converts the message array into an interator
    // The iterator has a method next() to return the next item in the sequence
    for (index, &item) in message.iter().enumerate() {
        // We can also enumerate like in Python
        println!("Item {} is {}", index, item);
        if item == "e" {
            // In order for this statement to work you need &item instead of just item
            break;
        }
    }

    // This range will be inclusive of the start value but exclusive of the end value
    for number in 0..5 {
        println!("Number is {}", number);
    }
}
