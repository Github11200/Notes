use std::mem;

struct Shuttle {
    name: String,
    crew_size: u8,
    propellant: f64,
}

fn main() {
    // The Box<T> data type lets you store data on the heap intead of on the stack
    // Boxes are considered a smart pointer since they provide extra functionality beyond references such as having
    // ownership of the data it poitns to, and when Box<T> goes out of scope it deallocates the heap memory

    let vehicle = Shuttle {
        name: String::from("Atlantis"),
        crew_size: 7,
        propellant: 835958.0,
    };

    println!(
        "vehicle size on stack: {} bytes",
        mem::size_of_val(&vehicle)
    );

    // Now we can crate a box and Box<Shuttle> tells Rust that we are going to hold a data type of Shuttle
    // When we use this box data type the program will allocate enough memory for this data type, and then move
    // it into the space that it had allocated, and this is not a copy operation, so the vehicle variable loses
    // ownership of the struct and will no longer be valid, so the box to the vehicle variable (on the heap)
    // becomes the new owner of the struct through the box pointer, which lives on the stack
    let boxed_vehicle: Box<Shuttle> = Box::new(vehicle);

    println!(
        "vehicle size on stack: {} bytes",
        mem::size_of_val(&boxed_vehicle)
    );

    // Currently the boxed vehicle pointers are as shown, boxed_vehicle ----> box containing data (on the heap) ----> string type for the name of the vehicle
    // Now if we pass in the boxed_vehicle to the size_of_val() method then it will give the size of the pointer on the stack, but we want it for the heap
    // and to do this we have to use the dereference operator which is represented with a * symbol, and when applied to a pointer it denotes the pointed to
    // location (like in C and C++)
    println!(
        "vehicle size on heap: {} bytes",
        mem::size_of_val(&*boxed_vehicle)
    );

    // Now if we want to move the boxed_vehicle from the heap back to the stack then we can dereference it, and this will pass ownership back to the
    // unboxed_vehicle variable
    let unboxed_vehicle: Shuttle = *boxed_vehicle;
    println!(
        "vehicle size on stack: {} bytes",
        mem::size_of_val(&unboxed_vehicle)
    );

    // Use cases for Box<T> type:
    // ----> Store a type whose size cannot be known at compile time, this can be useful for recursive types like a struct that has another struct as one of
    // it's fields (for a linked list for example)
    // ----> Another reason to use a box is to transfer ownership of data rather than copy it onto the stack, and this can be useful to avoid copying
    // large amounts of stack data because you can move it to the heap where ownership can be easily transferred
}
