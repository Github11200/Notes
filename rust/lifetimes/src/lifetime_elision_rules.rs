fn main() {
    let message = String::from("Greetings from Earth!");
    let first_word = get_first_word(&message);
    println!("first_word is {}", first_word);
}

// Before Rust version 1.0 you would have to annotate any references with a lifetime annotation as shown below, but then
// Rust developers realised that in some places the compiler could safely infer these lifetimes without the need of
// explicitly defining them. In order to do this they have the compiler the ability to see these patterns using Lifetime
// Elision Rules which are a set of rules for the compiler to analyze reference lifetimes, and if you code matches these
// patterns you can omit the explicit lifetime annotations. There are currently 3 lifetime elision rules:

//      1. Each input parameter that is a reference is assigned to it's own lifetime, so if a function has only one
//      reference parameter would need to have one lifetime: fn get_first_word<'a>(x: &'a str) -> &str {}. Remember that
//      references are only needed for references and if there are ny inputs that are not references you don't need
//      lifetimes for it. Also a function with two seperate references would get to lifetimes, 'a, and 'b, and so on for
//      however many parameters there are

//      2. If there is exactly one input lifetime, assign it to all output lifetimes, and this rule would apply to the
//      get_first_word function because it only has one input, so the return will also have the same lifetime as that input,
//      for this function, fn get_longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {} you would be required to have the
//      lifetime annotations since it doesn't meet this second rule because there are 2 lifetime annotations and we don't
//      know which one will be returned

//      3. If there is a &self or &mut self input parameter, it's lifetime will be assigned to all output lifetimes, so an
//      example is this function, fn send_transmission(&self, msg: &str) -> &str {}, and this function would be assigned
//      an input lifetime from rule number one, fn send_transmission<'a, 'b>(&'a self, msg: &'b str) -> &str {}, and
//      then if we apply this third rule then the output lifetime will be that of &self so the function lifetimes become
//      fn send_transmission<'a, 'b>(&'a self, msg: &'b str) -> &'a str {}, and this is meant to cover the case where a
//      method returns one of the fields from a struct, but if the reference that the struct is returning is coming from
//      somewhere else then the compiler will catch that and require you to include explicit lifetime annotations

// If after working through these rules the compiler still can't determine the output lifetime then you will have to require
// explicit annotations

fn get_first_word<'a>(s: &'a str) -> &'a str {
    let bytes = s.as_bytes();

    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..index]; // found a space!
        }
    }

    &s // no spaces found; input is a single word
}
