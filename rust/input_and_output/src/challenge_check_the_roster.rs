use std::env;
use std::fs;

fn main() {
    if env::args().len() < 2 {
        println!("The program requires 2 arguments");
        return;
    }

    let file_name = env::args().nth(1).unwrap();
    println!("File name: {file_name}");
    let name_to_search_for_in_the_roster = env::args().nth(2).unwrap();

    let roster_contents = fs::read_to_string(file_name).unwrap();

    for name in roster_contents.lines() {
        if name == name_to_search_for_in_the_roster {
            println!("The name {name_to_search_for_in_the_roster} was found in the roster!");
            return;
        }
    }

    println!("{name_to_search_for_in_the_roster} was not found in the roster.");
}
