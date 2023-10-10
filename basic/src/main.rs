use rand::Rng;
use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your Guess number.");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read Line");
    let a = 10;
    let mut b = 20;
    let c = &a + &b;
    println!("Your result is {c}");
    b = 30;
    println!("Your resutl is {c}");
    println!("Your guessd : {guess}");
}
