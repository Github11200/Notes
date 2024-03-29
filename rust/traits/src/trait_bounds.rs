use std::any;
use std::fmt;

fn print_type<T: fmt::Debug>(item: T) {
    // In order to be able to display the item inside this print macro it has to implement the Display trait, and for this
    // we have to explicitly set a bound to restrict the generic type T to data types  that implement Display, and this is
    // a part of Rust's std::fmt library. Then we can add T: fmt::Display to restrict it to items that implement this
    // Display trait
    println!("{:?} is {}", item, any::type_name::<T>());
}

fn main() {
    // When working with generics, you'll often need to use traits to serve as bounds to stipulate the functionality a type implements.
    // Bounding restricts a generic to only data types that form to those bounds so the functions can deal with those specific types
    print_type(13);
    print_type(13.0);
    print_type("thirteen");
    print_type([13]);
}
