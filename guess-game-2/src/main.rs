use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Hello to guess game {}", sum(1, 2));
    let secret_number = rand::thread_rng().gen_range(1..100);

    loop {
        let mut user_input = String::new();
        println!("Please type a number");

        io::stdin()
            .read_line(&mut user_input)
            .expect("Error getting user input");

        let user_input: u32 = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match user_input.cmp(&secret_number) {
            Ordering::Less => println!("It is smaller"),
            Ordering::Greater => println!("It is bigger"),
            Ordering::Equal => {
                println!("You won!");
                break;
            }
        }
    }
}
