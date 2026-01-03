use std::io;
use rand::Rng;

fn main() {
    println!("Guess a Number b/n 1 and 10!");
    let computer_guess = rand::rng().random_range(1..=10);

    println!("Please provide your guess!");
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Failed to read line");

    let user_guess: i32 = user_input.trim().parse().expect("Please type a number!");
    
    println!("You guessed {}", user_guess);
    println!("Computer guessed {}", computer_guess);

    if user_guess == computer_guess {
        println!("You won!");
    } else {
        println!("You lost!");
    }
}
