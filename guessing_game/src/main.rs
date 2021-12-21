// Imports for Random, Compare and In&Out 
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    // Generates a random number using the rand library and the single thread random number generator function
    let secret_number = rand::thread_rng().gen_range(1..101);
    // A continous loop that only ends till the correct number has been guessed
    loop {
        println!("Please input your guess.");
        // Saves the user input in a mutable String;
        let mut guess = String::new();
        // Checks if the user input is a valid input
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        // Trims the user input and checks if the input is a number if not it begins the loop from the beginning
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        // Output of what you guessed
        println!("You guessed: {}", guess);
        // checks if the guess matches with the secret number via the cmp function
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
