// When borrowing references that hold a sequence of elements like an array or string and we only
// want to reference a subset of those elements instead of the entire collection then we can create
// a slice. This references a contiguous section of a collection without taking ownership of it. The
// most popular slice is the string slice which is done using &str. String literals are slices which
// is hardcoded into the executable and the program uses a slice to access it.

fn main() {
    let message = String::from("Greetings from Earth!");
    println!("Message is {}", message);

    // Here we slice the message starting from the E in Earth to the exclamation mark
    // In this case message is the owner of the string, and last word just has a pointer
    // starting at the letter E. If we want our slice to go from E to the last letter
    // we can change 15..15 + 5 -> 15..
    let last_word = &message[15..15 + 5];
    println!("last_word is {}", last_word);

    // The length of string slices is in bytes and not in characters, so individual characters
    // may span multiple bytes since Rust is UTF-8 encoded. Also, rang indices must occur at
    // valid UTF-8 character boundaries, so if you slice it in the middle of a multi byte UTF-8
    // character then it will throw an error. All of this wasn't a concern in this program
    // because each letter takes up one byte, but if you have special characters or emojis which
    // will span multiple bytes.

    let planets = [1, 2, 3, 4, 5, 6, 7, 8];
    // The program creates a slice to the first 4 planets in the array, the &[i32] means the data
    // type is a borrowed reference to an array of i32 values.
    let inner_planets: &[i32] = &planets[..4];
    println!("inner_planets are {:?}", inner_planets);
}
