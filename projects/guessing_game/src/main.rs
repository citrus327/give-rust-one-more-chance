// https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();
    // let mut guess = "";

    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");

    let guess: u32 = guess.trim().parse().expect("to be a number");
    let secret = rand::thread_rng().gen_range(1..=100);

    println!("You guessed: {guess}");
    println!("The secret number: {}", secret);

    match guess.cmp(&secret) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
