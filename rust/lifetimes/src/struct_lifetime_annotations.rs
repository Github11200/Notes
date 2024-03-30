// When you have a struct that has a string it owns the string, so when the struct goes out of the scope the
// string will also be dropped out of memory

// Now if we change the name from a String type of &str it will give an error because the struct no longer owns
// &str and so it's not clear to the compiler how the lifetime of the borrowwed string relates to the lifetime
// of the struct, and if the string is dropped and dissapears while the struct is still in scope, then the struct
// tries to reference and use the no longer existent string would cause a problem. In order to stop this we have
// to add explicit lifetime annotations
struct Shuttle<'a> {
    name: &'a str,
}

// If we explicitly set the lifetime in the struct then we have to add <'a> after impl and the name
impl<'a, 'b> Shuttle<'a> {
    // Now we didn't have to add <'a> here and this is because of the third lifetime elision rule where if &self
    // is one of the parameters then the lifetime of the return type will be the same as that of &self

    // Now if we return msg instead of self.name then we will get an error, and to fix this we have to add an
    // explicit lifetime since rule number 3 no longer works since we're not returning a property of the struct
    fn send_transmission(&'a self, msg: &'b str) -> &'b str {
        println!("Transmitting message: {}", msg);
        msg
    }
}

fn main() {
    let vehicle = Shuttle { name: "Endeavour" };

    let sender = vehicle.send_transmission("Greetings from orbit!");
    println!("send is {}", sender);
}
