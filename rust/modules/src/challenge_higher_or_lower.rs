use rand::{thread_rng, Rng};
use std::io;

fn main() {
    let number_to_guess = generate_the_random_number_to_guess();
    let mut guess: u32 = 0;

    println!("Enter a number between 1 and 100: ");
    take_input(&mut guess);

    guessing_the_number(guess, number_to_guess);

    println!("You got it! The number was {number_to_guess}");
}

fn guessing_the_number(mut guess: u32, number_to_guess: u32) {
    while guess != number_to_guess {
        if guess > number_to_guess {
            println!("To high! Try again: ");
            take_input(&mut guess);
        } else {
            println!("To low! Try again: ");
            take_input(&mut guess);
        }
    }
}

fn generate_the_random_number_to_guess() -> u32 {
    let mut rng = thread_rng();
    let number_to_guess: u32 = rng.gen_range(1..=100);
    number_to_guess
}

fn take_input(guess: &mut u32) {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer);
    *guess = buffer.trim().parse().unwrap();
}
