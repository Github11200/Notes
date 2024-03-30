use rand::{thread_rng, Rng};
use std::io;

fn main() {
    let number_to_guess = generate_the_random_number_to_guess();
    let mut guess = 0;

    guessing_the_number(guess, number_to_guess);

    println!("You got it! The number was {number_to_guess}");
}

fn guessing_the_number(mut guess: u32, number_to_guess: u32) {
    loop {
        let result = take_input();

        if result == 0 {
            println!("\nFailed to parse input, Guess again:");
            continue;
        } else {
            guess = result;
        }

        if guess > number_to_guess {
            println!("To high! Try again: ");
        } else if guess < number_to_guess {
            println!("To low! Try again: ");
        } else {
            break;
        }
    }
}

fn generate_the_random_number_to_guess() -> u32 {
    let mut rng = thread_rng();
    let number_to_guess: u32 = rng.gen_range(1..=100);
    number_to_guess
}

fn take_input() -> u32 {
    let mut buffer = String::new();

    match io::stdin().read_line(&mut buffer) {
        Ok(_) => match buffer.trim().parse::<u32>() {
            Ok(value) => return value,
            Err(_) => return 0,
        },

        Err(_) => return 0,
    };
}
