mod hello_world;

use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let apples = 5; // immutable
    let mut bananas = 10; // mutable

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("Apples = {apples}, Bananas = {bananas}");
    println!("Apples + Bananas = {}", apples + bananas);

    println!("You guessed: {guess}");
}
