// We can also use generic types in functions as shown below
// We also have to add PartialOrd so that the function only accepts numeric data types that can be compared since
// they implement the PartialOrd trait
fn get_biggest<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

fn main() {
    println!("biggest is {}", get_biggest(1, 2));
}
