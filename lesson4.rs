use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, BufRead, Write};

const EASY_LEVEL: u32 = 20;
const MEDIUM_LEVEL: u32 = 50;
const HARD_LEVEL: u32 = 100;
const MAX_ATTEMPTS: u32 = 5;

fn main() {
    println!("Guess the number!");

    let level = select_level();
    let secret_number = rand::thread_rng().gen_range(1..=level);
    println!("Guess a number between 1 and {level}");

    let mut counter: u32 = 0;

    loop {
        println!("-------------------------------------------");
        print!("Your answer (or 'exit'): ");
        io::stdout().flush().unwrap();

        let input = read_line();
        if input.is_empty() {
            continue;
        }

        let trimmed = input.trim().to_lowercase();
        if trimmed == "exit" {
            println!("Game exited after {counter} attempts. Secret was {secret_number}");
            break;
        }

        let guess: u32 = match trimmed.parse() {
            Ok(num) if num >= 1 && num <= level => num,
            Ok(_) => {
                println!("Number must be between 1 and {level}");
                continue;
            }
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        counter += 1;

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small! Attempt: {counter}"),
            Ordering::Greater => println!("Too big! Attempt: {counter}"),
            Ordering::Equal => {
                println!("You win!");
                println!("You guessed it in {counter} attempts");
                break;
            }
        }

        if counter >= MAX_ATTEMPTS {
            println!("You lost :(. Secret was {secret_number}");
            break;
        }
    }
}

fn select_level() -> u32 {
    loop {
        println!("Choice level:");
        println!("1. Easy (1-{EASY_LEVEL})");
        println!("2. Average (1-{MEDIUM_LEVEL})");
        println!("3. Hard (1-{HARD_LEVEL})");

        print!("Level (1-3): ");
        io::stdout().flush().unwrap();

        let input = read_line();
        if input.is_empty() {
            continue;
        }

        match input.trim().parse::<u32>() {
            Ok(1) => return EASY_LEVEL,
            Ok(2) => return MEDIUM_LEVEL,
            Ok(3) => return HARD_LEVEL,
            _ => println!("Invalid choice! Please enter 1, 2, or 3."),
        }
    }
}

fn read_line() -> String {
    let mut input = String::new();
    match io::stdin().lock().read_line(&mut input) {
        Ok(_) => input,
        Err(_) => {
            println!("Input error, try again.");
            String::new()
        }
    }
}
