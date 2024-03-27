// When dealing with strings a borrowed reference to the string is not the same as a slice
// from the string. This means &String != &str
// This is because &String is a pointer that points to an actual string on the stack, which
// in turn is pointing to and owns a string data on the heap
// A slice, &str, only stores a pointer to the heap data along with length information since
// it is just borrowing the string and not actually owning it

// Rust will allow us to use a string reference, &String, in places that ask for a string
// slice, &str, this is known as Deref Coercion.
// &String ----> &str ----> String Data on Heap ✅
// This does not work the other way around though
// &str ----> &String ----> String::from() ----> String data on heap ❌

fn main() {
    let message = String::from("Greetings from Earth!");
    let first_word = get_first_word(&message[10..]);
    println!("first_word is {}", first_word);
}

// This function takes a borrowed reference of the string, and then returns a slice containing
// the first word of that input string without taking ownership of the string at all
// Now if we are passing in &message[10..] to the function then we need to change
// &String -> &str. If we pass in just &message then &str as the input will still work
fn get_first_word(s: &str) -> &str {
    // It first converts the input string into a slice of bytes representing the contents
    let bytes = s.as_bytes();

    // Then it iterates through the slice of bytes checking for the byte sequence that represents
    // a space character
    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..index]; // found a space!
        }
    }

    &s // no spaces found; input is a single word
}

// RULE OF THUMB: When writing functions that work with strings without taking ownership, use the
// string slice data type for input and output parameters because it has the flexibility to work
// with string references.
