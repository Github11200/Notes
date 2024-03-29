use std::fmt;

// Both types need to implement the Display trait in order for us to be able to print them
// In order to use multiple trait bounds we can use the + operator (also PartialEq is a part of the prelude so we don't need to import it)
// We also have to make sure that we can do T::from(b) to convert b to the data type of T, and for this we need to make sure that T has
// the trait bound of From<U>. Now when we call this from() method it will consume the variable b effectively moving it to a new variable
// of type T to use for the equality comparision. This means that when we try to use b in the print statements b will be unavailable
// because it was moved. In order to fix this the data type for b needs to implement the copy trait

// fn compare_and_print<T: fmt::Display + PartialEq + From<U>, U: fmt::Display + PartialEq + Copy>(
//     a: T,
//     b: U,
// )

// We can see in the function signature above that because of these trait bounds it got quite long with just 2 variable so to fix this
// Rust providesan alternate syntax using a where clause. The syntax is shown below with the where keyword and it does the exact same
// thing except it makes it easier to read especially when there are more than 2 inputs
fn compare_and_print<T, U>(a: T, b: U)
where
    T: fmt::Display + PartialEq + From<U>,
    U: fmt::Display + PartialEq + Copy,
{
    // Since we allow 2 different data types this if statement contains T::from(b) to convert the value of b into data type T
    if a == T::from(b) {
        println!("{} is equal to {}", a, b);
    } else {
        println!("{} is NOT equal to {}", a, b);
    }
}

fn main() {
    compare_and_print(1.0, 1);
    compare_and_print(1.1, 1);
}
