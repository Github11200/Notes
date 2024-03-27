use rand;

fn main() {
    // Crates in Rust is a collection of Rust source code files, and Binary crates compile to produce an
    // executable program, and the other type of crates are called Libraries which contain code for other
    // programs to use (like an NPM package). Crates.io is the official registery for crates. In order to
    // use crates we have to add them to the cargo.toml file, so we can go to Crates.io fine the crate,
    // and then add something like rand="0.8.0" to the cargo.toml file (this crate is for random numbers).
    // We add this crate under the [dependencies] part of the cargo.toml file. Now if we want to use the crate
    // then we simply just do use rand;
    let number = rand::random::<f64>(); // when we do random::<f64>() it is called a turbo fish operator
    println!("number is {}", number);

    // Now if we wanted to use just the random function then we can change use rand ----> use rand::random, and
    // then we don't have to write rand::random::<f64>() instead we can just do random::<f64>(), now if you
    // do this then it might cause errors if you also have a function called random because then there will be
    // an error.

    // The rand crate also has it's own prelude to bring in it's most common elements so we can do something
    // like use::rand::prelude::* where the * means wild card and it will bring in everything inside the rand
    // prelude.
}
