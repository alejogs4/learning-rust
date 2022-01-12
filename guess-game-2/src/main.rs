use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut user_guess = String::new();
        println!("Please enter your guess");

        io::stdin()
            .read_line(&mut user_guess)
            .expect("Error reading guess!");

        let user_guess: u32 = match user_guess.trim().parse() {
            Ok(number) => number,
            Err(_) => continue,
        };

        match user_guess.cmp(&secret_number) {
            Ordering::Greater => println!("It is bigger than expected"),
            Ordering::Less => println!("It is smaller than expected"),
            Ordering::Equal => {
                println!("You won!");
                break;
            }
        }
    }
}
