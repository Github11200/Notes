use std::fs;
use std::io::prelude::*;

fn main() {
    let mut speech = String::new();
    speech.push_str("We choose to go to the Moon in this decade\n");
    speech.push_str("and do the other things,\n");
    speech.push_str("not because they are easy,\n");
    speech.push_str("but because they are hard.\n");

    // We can use the fs::write() method to write to a file, and it takes two arguments, one is the path
    // to the file, and the second are the contents we want to write to it
    fs::write("speech.txt", speech);

    // The fs::write() method will completely replace the contents of existing files, and it also writes
    // what you give it as the entire content of the file all at once, so if you need to write multiple
    // pieces to a file then it doesn't work very well

    // Now if we want to append data instead of replacing it then we can use fs::OpenOptions::new(), this will
    // create a blank set of options that we can use to configure how the file will open, and since we want
    // to append data we will call the append() method and pass an argument of true and then the open argument
    // then the path of the file we wnat to open, and since this returns the error handling enum type we have
    // to use the unwrap() method to get the contents
    let mut file = fs::OpenOptions::new()
        .append(true)
        .open("planets.txt")
        .unwrap();

    // Now if we want to actually write contents to the file then we have to import std::io::prelude::* and
    // this lets us import a lot of the popular items all together. Inside this we have the std::io::Write
    // module which is a trait but there are other traits such as the Read trait

    // Here we're passing a a string literal but the funciton doesn't care about what the data represents,
    // it just thinks of it as a series of bytes and it expects the argument to be an array of u8 values,
    // so we have ot mark thsi string literal as a collection of byte values by adding a lowercase b prefix
    file.write(b"\nPluto");
}
