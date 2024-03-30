fn main() {
    // Here we're going to be talking about runtime errors, not compilation errors, and Rust groups these errors
    // into two categories, recoverable and unrecoverable. Recoverable errors are something the program can do
    // something, like if a user opens an incorrect file then the program can at least display a message about
    // that. Unrecoverable errors are ones like indexing beyond array bounds

    // Rust does not have exceptions in the traditional programming sense instead recoverable errors are handled
    // with the Result<T, E> enum type, and the panic! macro for unrecoverable errors. When you call the panic
    // macro it causes the program to terminate and provide feedback

    // We can manually create a panic as shown below
    // panic!("Houston, we've had a problem.");

    // This will cause the program to panic when it tries to divide 0 by 0
    let countdown = [5, 4, 3, 2, 1, 0];

    for count in countdown.iter() {
        println! {"T-minus {}", count};
        let x = 1 / count; // this won't end well
    }
}
