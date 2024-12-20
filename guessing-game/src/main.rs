use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);
    println!("You guessed the secret number {secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("TODO: panic message");

        let guess: u32 = guess.trim().parse().expect("Failed to parse");

        match guess.cmp(&secret_number) {
            Ordering::Equal => println!("You Won"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Less => println!("Too Small"),
        }

        println!("Ok Bye");
    }
}
