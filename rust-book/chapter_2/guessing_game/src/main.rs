use std::io;

fn main() {
    println!("guess the number!");
    println!("input your guess.");

    let mut guess = String::new(); // growable, utf-8

    io::stdin().read_line(&mut guess)
        .expect("failed to read line.");
    
    println!("you guessed {}", guess);
}