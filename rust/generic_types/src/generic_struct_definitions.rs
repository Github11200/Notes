#[derive(Debug)]
// To create a generic add <T> and then use that as the data type
struct Rectangle<T, U> {
    width: T,
    height: U,
}

fn main() {
    // Generic data types are abstract stand-ins for concrete data types or other properties, and can be used
    // with structs, functions, methods, and more. This will let us define one Rectangle struct to define
    // multiple types for it. It is defined with <T>

    // If we make the first property in this definition a u8, and then the second u16 it will give an error because
    // it sees the width property first and substitutes it's data type in for T, to fix this we can add another type
    // which in this case is U, <T, U>, and then that will be another data type
    let rect = Rectangle {
        width: 1u8,
        height: 3u16,
    };

    // Generics are a zero-cost abstraction so it won't slow down your code. The compiler uses monomorphization
    // to replace generic placeholders with concrete data types

    println!("rect is {:?}", rect);
}
