use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    // Generate a random number locked to the host operating system's thread.
    // Note that gen_range is inclusive on the lower bound but exclusive on the
    // upper bound, therefore this generates a random number between 1 and 100.
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        // The mut keyword makes a mutable variable.
        // Variables are immutable by default.
        let mut guess = String::new();

        // Read input from stdin and store it in our mutable variable.
        // read_line returns a ✨Result✨ enumeration (Ok, Err). We can invoke
        // the .expect() function on the result to crash the program with a
        // message if the result is Err.
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Trim and parse the input to an unsigned 32-bit integer.
        // Note that parse() also returns a Result. We handle the enum cases
        // explicitly here.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        // Compare the parsed input to the secret number using the cmp()
        // function, which returns an ✨Ordering✨ enum.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win! 🦀");
                break;
            }
        }
    }
}
