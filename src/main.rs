use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guessing game. Guess the number.");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    let mut num_guesses: u32 = 0;

    loop {
        let mut guess = String::new();

        println!("Please input your guess (between 1 and 100).");
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

        let guess = guess.trim();

        if guess == "q" || guess == "quit" {
            println!("Game was quit.");
            break;
        }

        let guess: u32 = match guess.parse() {
            Ok(num) => {
                num_guesses += 1;
                num
            },
            Err(_) => continue,
        };

        println!("Your guess was {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("Well done! Completed in {} guesses.", num_guesses);
                break;
            },
            Ordering::Less => println!("Too small."),
            Ordering::Greater => println!("Too big."),
        }
    }
}
