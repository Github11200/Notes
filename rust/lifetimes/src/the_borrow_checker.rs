fn main() {
    let propellant;
    {
        let rp1 = String::from("RP-1");
        propellant = &rp1;
    }
    // If we put the print statement outside then it will give an error because rp1 will have gone out of scope,
    // to fix this we have to move let rp1 = String::from("RP-1") before the opening curly brace in order for it
    // to still be available outside of that sub scope
    println!("propellant is {}", propellant);

    // Under the hood Rust uses the borrow checker which compares the scopes of variables to make sure whether all borrows
    // are valid
}
