use std::fs;
use std::io;

fn read_and_combine(f1: &str, f2: &str) -> Result<String, io::Error> {
    // You can only use this question mark shorthand in functions that return a result enum, and it
    // is a shorthand for what is shown with the match operator in s2
    let mut s1 = fs::read_to_string(f1)?;

    let s2 = match fs::read_to_string(f2) {
        Ok(s) => s,
        Err(e) => return Err(e),
    };

    s1.push('\n');
    s1.push_str(&s2);
    Ok(s1)
}

fn main() {
    let result = read_and_combine("planets.txt", "dwarf_planets.txt");

    match result {
        Ok(s) => println!("result is...\n{}", s),
        Err(e) => println!("There was an error: {}", e),
    };
}
