// If we change this function to always return x no matter what and still leave the input parameter y as y: &'a str then it
// will still give an error because the annotation specifies that x, y, and the return reference will all have the same
// generic lifetime, and that's what the borrow checker will enforce, but since the lifetime of y doesn't have anything to
// do with th return value we don't actually need it so we can change y: &'a str ----> y: &str. Even if we change it to this
// it might be confusing for someone else reading the code to figure out what we intended and whether we just forgot to add
// the lifetime annotation for y. To make things more explicit we can add 'b as another lifetime annotation
fn best_fuel<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        x
    }
}

fn main() {
    let result;
    let propellant1 = String::from("RP-1");
    {
        let propellant2 = String::from("LNG");
        result = best_fuel(&propellant1, &propellant2);
    }
    println!("result is {}", result);
}
