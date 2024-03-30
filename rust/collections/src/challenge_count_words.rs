use core::num;
use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    if env::args().len() < 2 {
        println!("Please specify the name of the file you would like to use, eg. planets.txt");
        return;
    }

    let file_path = env::args().nth(1).unwrap_or(String::from(""));
    if file_path == "" {
        println!("Please specify a proper file path.");
        return;
    }

    let result = match fs::read_to_string(file_path) {
        Ok(file_content) => file_content,
        Err(error) => {
            panic!("There was an error reading the file: {error}");
        }
    };

    let binding = result.to_lowercase();
    let contents = binding.split_whitespace();

    let mut words = HashMap::new();

    for byte in contents {
        let word = words.entry(byte).or_insert(0);
        *word += 1;
    }

    let all_words = Vec::from_iter(words.keys().cloned());
    let number_of_times_showed_up = Vec::from_iter(words.values().cloned());

    let mut max_value = 0;
    let mut indexes: Vec<i32> = vec![];

    let mut i = 0;

    for number in number_of_times_showed_up {
        if number > max_value {
            indexes.clear();
            indexes.push(i);
            max_value = number;
        } else if number == max_value {
            indexes.push(i);
        }

        i += 1;
    }

    println!("Most common word(s) are:");
    for index in indexes {
        println!("{}", all_words[index as usize]);
    }
    println!("This word showed up {} times", max_value);
}
