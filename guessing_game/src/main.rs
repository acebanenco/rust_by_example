use std::{env, io};
use std::cmp::Ordering;

use rand;
use rand::Rng;

fn main() {
    let args:Vec<String> = env::args().collect();
    let use_suggest;
    if args.len() > 1 {
        use_suggest = args.into_iter()
            .find(|arg| arg.as_str().eq("--suggest"))
            .is_some();
    } else {
        use_suggest = false;
    }

    println!("Guess the number!");

    let mut guess_min = 1;
    let mut guess_max = 20;

    let max_attempts = ((guess_max - guess_min + 1) as f64).log2().ceil() as u32;

    let secret_number = rand::thread_rng()
        .gen_range(guess_min..guess_max + 1);

    let mut attemt = 1;
    loop {
        if attemt > max_attempts {
            println!("You lost!");
            break;
        }

        let suggested_guess = guess_min + (guess_max - guess_min) / 2;
        println!("Attempt {} of {}", attemt, max_attempts);
        if use_suggest {
            println!("Please input your guess [{}]: ", suggested_guess);
        } else {
            println!("Please input your guess: ");
        }

        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => suggested_guess
        };
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small!");
                guess_min = guess;
            }
            Ordering::Greater => {
                println!("Too big!");
                guess_max = guess;
            }
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
        attemt += 1;
    }
}
