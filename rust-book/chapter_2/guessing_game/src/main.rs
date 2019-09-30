use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("guess the number!");
    let secret: u32 = rand::thread_rng().gen_range(1, 101);
    loop {
        println!("input a guess...");
        let mut guess = String::new(); // growable, utf-8
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line.");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(error) => {
                println!("Error with input: {}", error);
                continue;
            }
        };
        match guess.cmp(&secret) {
            Ordering::Less => println!("Too low"),
            Ordering::Greater => println!("Too high"),
            Ordering::Equal => {
                println!("Success!");
                break;
            }
        }
    }
}
