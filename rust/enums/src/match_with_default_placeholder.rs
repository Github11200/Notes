fn main() {
    // You can also use the match operator on other variables that are not enums

    let my_number = 4u8;

    // An important thing to remember is that the compiler goes from top to bottom and returns the first match, so if we had the wildcard
    // operator, _, where the number 1 is and my_number was 3 then it won't print out the value of 3 because it already matched it with
    // the wildcard operator
    let result = match my_number {
        0 => "zero",
        1 => "one",
        2 => "two",
        3 => "three",
        _ => {
            // This underscore is a wildcard, or default case, because we don't want to type out the cases for all 256 possible numbers
            println!("{} did not match", my_number);
            "something else"
        }
    };

    println!("result is {}", result);
}
