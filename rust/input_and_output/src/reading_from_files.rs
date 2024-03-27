use std::fs;

fn main() {
    // Rust has a fs module for manipulating the file system, and to use it we have to import it,
    // in this program we will be reading from the planets.txt file

    // This function returns and enum type for error handling, but for now we'll just use the unwrap() method
    let contents = fs::read_to_string("planets.txt").unwrap();
    println!("contents is {}", contents);

    // Now to go through line by line we can use the lines iterator
    for line in contents.lines() {
        println!("line is {}", line);
    }

    // We may also have to read file types with images or vidoes and we have to work with the data as
    // a sequence of bytes, in order to do this we can use the fs::read() method, and this will return
    // a vector of u8 values representing the individual bytes in the file
    let contents = fs::read("planets.txt").unwrap();
    // We also have to use the debug formatter {:?} to print out these bytes
    println!("contents is {:?}", contents);

    // We used a string literal to store the path to the planets.txt file, but if you are writing programs
    // that work with more complex paths that need to work on multiple operating systems then you should
    // use the std::path module which provides cross platform support for manipulating paths
}
