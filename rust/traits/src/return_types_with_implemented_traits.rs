use std::fmt;

// If we create an if statement inside this function based on an input parameter then it will give an error
// because the Rust compiler needs to determine the actual data type to be returned so it can return and
// execute the correct code, and the subroutines to display an integer value is probably a bit different
// than the subroutine to display a string literal, and the rust compiler doesn't allow this kind of
// abiguity. If you need to work with return types that cannot be known until runtime then that gets into
// the topic of dynamic dispatch
fn get_displayable(choice: bool) -> impl fmt::Display {
    if choice {
        13
    } else {
        "thirteen"
    }
}

fn main() {
    // Now let's look at how we can use traits for setting bounds on return values, and in this example
    // we will have a function that only returns values with a Display trait

    println!("output is {}", get_displayable(true));
}
