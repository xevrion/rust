use std::cmp::Ordering;
use std::io;

use rand::Rng; // random number generator

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        // in rust, variables are immutable by default, so we make them mutable whenever we need them to be!

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        // by default even references are immutable, so rather than `&guess` you have to write `&mut guess` to make them mutable.

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        // now we compare
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
