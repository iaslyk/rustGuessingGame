use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    let min = 1;
    let max = 100;
    println!("Guess the number between {min} and{max}!");
    let secret_number = rand::thread_rng().gen_range(min..=max);
    let mut count = 1;

    loop {
        println!("Please input your guess");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => { num }
            Err(_) => continue
        };
        println!("You guessed: {guess}");
        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small");
                count = count + 1
            }
            Ordering::Greater => {
                println!("Too large");
                count = count + 1
            }
            Ordering::Equal => {
                println!("You win!");
                println!("You needed {count} tries!");
                break;
            }
        }
    }
}