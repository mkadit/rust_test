use rand::Rng;
use std::{cmp::Ordering, io}; // Multiple imports
use text_io::read;

pub fn run() {
    // Create a mutable variable with an empty String
    println!("What is your name?");
    let mut input = String::new();

    // Get input using non-crate
    io::stdin() // Call function
        .read_line(&mut input) // Handle it, read line of input, assign to buffer
        .expect("Failed to read line"); // Error message

    println!("Greetings {}", input);

    println!("Guess the number: ");

    // Generate Random number from low (inclusive) to high (exclusive)
    let secret_num = rand::thread_rng().gen_range(1, 101);

    // Using shadow reassign input to a new empty string
    loop {
        let mut input = String::new();
        // Get input using non-crate
        io::stdin() // Call function
            .read_line(&mut input) // Handle it, read line of input, assign to buffer
            .expect("Failed to read line"); // Error message

        // Convert the input to an int32 and assign it to num_guess
        // trim() for trimming the whitespace
        // parse() to parse it to another type
        // Expect is the message that will be write when it's successfully done
        let num_guess: i32 = input.trim().parse().expect("Your guess?");

        println!("You guessed: {}", input);
        match num_guess.cmp(&secret_num) {
            Ordering::Less => println!("Cold! Blue Cold!"),
            Ordering::Greater => println!("Hot! Red Hot!"),
            Ordering::Equal => {
                println!("That's the spot!");
                break;
            }
        }
    }

    println!("The secret number is: {}\n", secret_num);

    // Using shadow reasign num_guess
    println!("Type something fun!");
    // Input using text_io crate and make sure it's type is String
    let num_guess: String = read!();
    println!("Using the power of shadow num_guess become {}!", num_guess);
}
