use std::env;

// Command-line arguments let us pass arguments into the program when it is invoked
// These are often used to pass file paths or configuration settings

fn main() {
    // If the user doesn't give all the arguments then the program will crash so we have to check that
    // with a simple if statement at the start of the program
    if env::args().len() <= 2 {
        println!("Program requires at least 2 arguments.");
        return;
    }

    // To take in command line arguments in rust we use the args function which is included in
    // std::env::args. Args will return an iterator over arguments passed into the program,
    // the first argument is usually the path to the executable.

    for (index, argument) in env::args().enumerate() {
        println!("argument {} is {}", index, argument);
    }

    // Now to pass in arguments to the program from the command line we can use the command
    // cargo run Moon 1961 --flag, and this will print out the output below:
    /*
        argument 0 is target\debug\input_and_output.exe
        argument 1 is Moon
        argument 2 is 1961
        argument 3 is --flag
    */

    // Now if we want a specific argument from the argsIterator, one way to do that is to use the
    // nth method which returns the nth item from an iterator
    let arg2 = env::args().nth(2).unwrap();
    println!("arg2 is {}", arg2);
    // Now if we run this with the same command as before then arg2 is 1961, and this is a STRING
}
