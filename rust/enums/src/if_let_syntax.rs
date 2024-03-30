fn main() {
    let number = Some(13);

    // Rust provides an easier way then this to check an Option enum
    // match number {
    //     Some(13) => println!("thirteen"),
    //     _ => (),
    // }

    // This is called a if let statement
    if let Some(13) = number {
        println!("thirteen");
    }
}
