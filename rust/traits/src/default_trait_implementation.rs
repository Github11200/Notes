// This is how we can derive traits, and now we can compare hubble == gps, and now if we want to do hubble > gps we also need
// to derive the PartialOrd trait
#[derive(PartialEq, PartialOrd)]
struct Satellite {
    name: String,
    velocity: f64, // miles per second
}

fn main() {
    // When you define a new custom data type with a struct, by default, the new struct does not implement any traits, so you have
    // to explicitly give it the traits it needs.

    // By default the Satellite struct doesn't implement the std::cmp::PartialEq trait which is needed when comparing two structs
    // like hubble and gps in this case, and we need to to do something like hubble == gps

    // Now if we want to use this PartialEq trait we can either define the required methods for it to make it easier and simply
    // derive that functionality. The rust compiler is able to provide a basic implementation for a handful of common traits
    // via the derived attribute, and when you do this the compiler will generate default code for teh requried methods
    // Derivable traits include Eq, PartialEq, Ord, ParitalOrd, Clone, Copy, Hash, Default, and Debug

    let hubble = Satellite {
        name: String::from("Hubble Telescope"),
        velocity: 4.72,
    };

    let gps = Satellite {
        name: String::from("GPS"),
        velocity: 2.42,
    };

    println!("hubble == gps is {}", hubble == gps);

    // The way the > operator works here is it goes through each property, in order, and checks which one is greater, and after it
    // finds just 1 property that is greater it will exit, or if it reaches the end it will return false
    println!("hubble > gps is {}", hubble > gps);
}
