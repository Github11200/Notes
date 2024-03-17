fn main() {
    // char represents a single character, in Rust it is a unicode scalar value, and all characters are stored using 4 bytes
    let letter = "a";
    let number = "1";

    // Because rust uses unicode characters it means that you can use a lot of different symbols and not just letters
    let finger = "\u{261D}"; // We can put it's hexadecimal value in curly braces, this variable is a finger pointing up, this image will not be shown in the command prompt though, only in the termianl here
    println!("{}\n{}\n{}", letter, number, finger)
}
