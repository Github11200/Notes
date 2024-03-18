/*
    If you do not specify what a function will return then Rust will implicitly return
    the unit data type. This data type is used when there is no other meaningful value to
    return. It is represented with (), you can also explicitly add this if you want to say
    that the function will not return anything, but the Rust compiler can also implicitly
    do this for you.
*/

fn main() {
    let result = square(13);
    let tuple_result = sqaure_two(13); // We can also destructure this tuple for other usages
    println!("result is {}", result);

    println!("tuple result is {:?}", tuple_result); // We can insert :? to tell the println! function to use a special debugging format instead
}

fn square(x: i32) -> i32 {
    println!("square {}", x);
    // If we leave the last line in a function without a semicolon and just an expression (something that returns a value
    // like 2 + 1 and not a statement like let x = 3) then it will automatically return that value
    x * x

    // We can also return it using a statement by writing, return x * x;
}

// We can also return a tuple as shown in the code below
fn sqaure_two(x: i32) -> (i32, i32) {
    println!("squaring {}", x);
    return (x, x * x);
}
