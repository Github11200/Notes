use std::io;

fn main() {
    // The standard input lets a user input into the command line, it is included in the io module, std::io
    let mut buffer = String::new();
    println!("Enter a message:");
    io::stdin().read_line(&mut buffer); // This will update the buffer when reading the line
    println!("buffer is {}", buffer);
}
