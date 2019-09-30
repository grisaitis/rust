use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret: u32 = rand::thread_rng().gen_range(1, 101);
    loop {
        let mut guess = String::new();
        println!("input guess:");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failure to read from stdin");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(message) => {
                println!("Error: {}", message);
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
    println!("Goodbye, world!");
}
