use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Please input your guess!");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let secret = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret}");

    println!("You guessed: {guess}");

    let guess: u32 = guess.trim().parse().expect("Please enter a number");
    match guess.cmp(&secret) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big"),
        Ordering::Equal => println!("You win"),
    }
}
