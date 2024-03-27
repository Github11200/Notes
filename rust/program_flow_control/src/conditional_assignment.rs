fn main() {
    let make_x_odd: bool = true;
    let x: i32 = if make_x_odd { 1 } else { 2 }; // Shorthand for if else statements

    /*
    if make_x_odd {
        x = 1;
    } else {
        // If we comment this out then Rust will throw a warning because x may be uninitialized
        x = 2;
    }*/

    println!("x is {}", x);
}
