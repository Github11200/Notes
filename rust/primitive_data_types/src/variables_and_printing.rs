fn main() {
    let a = 10;
    let b = 3.0;
    let c = a as f64 / b; // you can typecast a as float 64 because otherwise you can't divide these numbers

    println!("c is {}\na is {}", c, a); // you can print things out using the {} as a placeholder, and you can pass in the positional arguments as we have done with the a and c

    // You can also specify the number of decimal points you want to print to using :.3f with any number
    // You can also say the number of characters you want to print, so if you have :8.3f it will print out 8 characters and add spaces in the front to make it 8 characters
    /*
        c is 3.333
        c is  3.333
        c is   3.333
        c is    3.333
        c is     3.333
        c is      3.333
    */
    // If you don't want just spaces at the front then you can do :08.3f which will print out zeroes instead of spaces to add in the 8 characters
    println!("c is {:.3}", c);
    println!("c is {:4.3}", c);
    println!("c is {:5.3}", c);
    println!("c is {:6.3}", c);
    println!("c is {:7.3}", c);
    println!("c is {:8.3}", c);
    println!("c is {:9.3}", c);
    println!("c is {:10.3}", c);
    println!("c is {:08.3}", c);
}
