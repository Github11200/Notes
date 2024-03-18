fn main() {
    say_hello();
    say_hello();

    // We haven't specified the datatype for x and y, and by default they should be i32, and when we're passing them into the function they
    // aren't getting casted automatically unless you explicitly say so. Instead, the compiler is smart enough to know that x and y are being
    // passed into the funcion that requires parameters u8 so it will by default make x and y a type of u8 instead of the default integer type of i32
    // We can see this in action if we call say_a_number and pass x into it, this would result in an error, but we can cast x as an i32
    let x = 1;
    let y = 2;

    say_the_sum(x, y);
    say_a_number(x as i32);
}

// Rust doesn't care about where you define your functions, tehy don't have to be before the main function to call it
fn say_hello() {
    println!("Hello!");
    say_a_number(13);
}

fn say_a_number(number: i32) {
    println!("number is {}", number);
}

fn say_the_sum(a: u8, b: u8) {
    let sum = a + b;
    println!("sum is {}", sum);
}
