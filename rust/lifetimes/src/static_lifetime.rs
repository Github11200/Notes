struct Shuttle<'a> {
    name: &'a str,
}

impl<'a, 'b> Shuttle<'a> {
    fn send_transmission(&'a self, msg: &'b str) -> &'b str {
        println!("Transmitting message: {}", msg);
        msg
    }
}

fn main() {
    // There is a special lifetime in Rust which is the 'static lifetime, and this indicates that references
    // are available for the netire duration of the program so it will never get dropped. As an example the
    // text in a string literal is stored in the program's binary so it never goes away and you can annotate
    // it as shown, let s: &'static str = "Greetings from Neptune!"

    //However, references that have a static lifetime such as string literals can be coerced to other more
    // restrictive lifetimes if that reference is passed as the output parameter from a function with another
    // lifetime annotation, though that's rarely done

    // The 'static lifetime may also be used as a trait bound when defining generic data types to ensure that
    // the data has a static lifetime so the receiver can hold onto it and use it as long as they want knowing
    // it will never become invalid. Here is an example, T: Display + 'static and this means that in addition
    // to implementing the Display trait the generic type T includes the static lifetime which indicates that
    // whatever data type T is it won't contain any non-static references

    let vehicle = Shuttle { name: "Endeavour" };

    let sender = vehicle.send_transmission("Greetings from orbit!");
    println!("send is {}", sender);
}
