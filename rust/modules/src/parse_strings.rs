use std::io;

fn main() {
    // The standard input lets a user input into the command line, it is included in the io module, std::io
    let mut buffer = String::new();
    println!("Enter a message:");
    io::stdin().read_line(&mut buffer); // This will update the buffer when reading the line
    println!("buffer is {}", buffer);

    // Now if we want to take this input and interpret it as a different data type then we need to parse it.
    // If we want the user to input a number and use it as a numeric value then we have to convert the input string.
    // The string we get from standard input will have a new line at the end so we have to trim it.
    // When we use the .parse() method we have to specify the data type so we can either do it using angled
    // brackets, like .parse::<i32>() or we can add the data type to the variable let number: i32 = ...
    // When dealing with the parse method it will return an enum because we don't know what the user will return,
    // and the enum we get back will either have the i32 value or an error, but for now we'll just use the
    // unwrap() method
    let number: i32 = buffer.trim().parse().unwrap();
    println!("number + 1 is {}", number + 1);
}
