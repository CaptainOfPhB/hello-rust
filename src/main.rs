use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    println!("Enter your guess: ");

    let secret_number = rand::thread_rng().gen_range(0..100);

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    match guess.cmp(&secret_number) {
        Ordering::Equal => println!("You win!"),
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
    }

    println!("You guessed: {guess}");
}
