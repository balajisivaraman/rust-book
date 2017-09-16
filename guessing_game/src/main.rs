extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    let mut guesses = 0;
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        const GUESSES_ALLOWED: i32 = 5;

        io::stdin().read_line(&mut guess).expect(
            "Failed to read line",
        );

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number!");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small!");
                guesses = guesses + 1;
                if guesses < GUESSES_ALLOWED {
                    println!("you have {} guesses left", GUESSES_ALLOWED - guesses);
                } else {
                    println!("You have exhausted your guess limit. Game ends now.");
                    break;
                }
            }
            Ordering::Greater => {
                println!("Too big!");
                guesses = guesses + 1;
                if guesses < GUESSES_ALLOWED {
                    println!("you have {} guesses left", GUESSES_ALLOWED - guesses);
                } else {
                    println!("You have exhausted your guess limit. Game ends now.");
                    break;
                }
            }
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
