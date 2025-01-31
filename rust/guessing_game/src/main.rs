use rand::Rng;
use std::io;

fn main() {
    guessing_game();
}

fn guessing_game() {
    println!("Guess the number ");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Please input the number");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    if secret_number == guess.trim().parse().expect("Please type a number") {
        println!("You win!");
    } else {
        println!("You lose! The number was {}", secret_number);
    }
}
