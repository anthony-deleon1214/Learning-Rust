use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess) // & indicates a reference
            .expect("Failed to read line."); // Expect needed to handle Result value

        let guess: u32 = match guess.trim().parse() { // Changing to a match expression for error handling
            Ok(num) => num,
            Err(_) => continue, // Underscore is a catchall value so any Err information continues
        };

        println!("You guessed: {guess}");

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
