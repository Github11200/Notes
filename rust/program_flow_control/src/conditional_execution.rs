fn main() {
    let x: i32 = 3;

    if x == 3 {
        println!("x is 3!");
    }

    if x + 1 == 3 {
        println!("x + 1 is 3!");
    }

    // You can't do this in Rust because it expect a boolean value and not an integer value
    // if x {}
}
