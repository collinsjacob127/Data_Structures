/* Jacob Collins
 * Rust guessing game for practicing io and strings
 * August 7, 2022
 */
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            // trim() removes
            Ok(num) => num,     // whitespace and \n
            Err(_) => continue, // from terminal
        }; // input (when you
           // press enter)

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Large!"),
            Ordering::Equal => {
                println!("Too Perfect!");
                break;
            }
        }
    }
}
