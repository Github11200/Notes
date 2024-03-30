fn main() {
    // Many programming languages use a null value to indicate "no value", but this is not safe
    // because errors can occur when usign a null value in a not-null context. In Rust you use
    // a generic Enum named Option which can be one of two variants, if it's the Some variant
    // it means that's not null and does have a value which is stored in the Enum, but if the
    // enum is None value then it means it has no value inside it. This Option enum is also very
    // commonly used so it is included in the prelude. To instantiate a variable with something
    // you can do, let something = Some(13); and instantiate with nothing you can have a variable
    // be, let nothing = None;

    let countdown = [5, 4, 3, 2, 1];

    // We can use the get method to get an Option enum at the specified index, whereas if we had tried
    // to do this with square brackets it would have given an error
    let number = countdown.get(4);

    // While this piece of code does work it is discouraged to use the .unwrap() method because the option
    // enum may be None, and the program may compile but it will crash if we did .get(5) instead of 4.
    // A safer option is to use unwrap_or() which takes an input parameter and returns that instead if the
    // number is set to None. We also need to add the borrow operator to make it a reference since we need
    // to match the data type of the option enum which will be a reference to an integer within an array
    let number = number.unwrap_or(&0) + 1;

    println!("number is {:?}", number);
}
