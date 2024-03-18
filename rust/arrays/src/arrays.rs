fn main() {
    let mut letters = ["a", "b", "c"];
    letters[0] = "x";
    let first_letter = letters[0];
    println!("first_letter is {}", first_letter);

    // You can specify the data type of an integers array with 5 values like this
    let numbers: [i32; 5];
    // numbers = [0, 0, 0, 0, 0]; // you can initialize an array like this but it's tedious
    numbers = [1; 5]; // this is a faster way of doing so

    // Arrays in Rust have to be indexed usign usize, the size of this type will be decided by the compiler
    // For example, on a 32 bit processor the usize will be 4 bytes, but on a 64 bit process usize would be 8 bytes long
    let index: usize = numbers.len() - 1;

    println!("last number is {}", numbers[index]);
}
