use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let mut rng = rand::rng();

    let secret_number = rng.random_range(1..=100);

    let mut no_of_trials = 10;

    while no_of_trials > 0 {
        println!("Please input your guess ({no_of_trials} tries left):");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Convert input from string to number.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        no_of_trials -= 1;

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("You win!");
                return;
            }
            Ordering::Greater => println!("Too big!"),
        }
    }

    println!("Out of trials. The secret number was: {secret_number}");
}
