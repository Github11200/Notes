fn main() {
    // There are three loops in Rust, loop, while, and for
    let mut count = 0;

    let result = loop {
        if count == 10 {
            // Use the break keyword to break out of the code. We can
            // also return a value here and assign it to a variable
            break count * 10;
        }
        count += 1;
        println!("count is {}", count);
    };

    println!("After the loop!");
    println!("Result is {}", result);
}
