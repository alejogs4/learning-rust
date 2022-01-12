use std::io;

fn main() {
    // Variables
    let x = 10;
    println!("x = {}", x);
    // Variable shadowing
    let name = "Alejandro";
    let name = match name.len().checked_add(2) {
        Some(number) => number,
        None => name.len(),
    };
    println!("Name size = {}", name);

    // tuples
    let first_one: (u32, u32, char) = (1, 2, 'a');
    let (x, y, tag) = first_one;
    println!("x = {}, y = {}, tag = {}", x, y, tag);

    let x = first_one.0 + 1;
    let y = first_one.1;
    let tag = first_one.2;
    println!("x = {}, y = {}, tag = {}", x, y, tag);

    let numbers = [1, 2, 3, 4, 5];
    let mut user_index = String::new();

    io::stdin()
        .read_line(&mut user_index)
        .expect("Error reading user index");

    let user_index = match user_index.trim().parse() {
        Ok(number) => number,
        Err(_) => 0,
    };

    let selected_index = match user_index {
        0..=4 => user_index,
        _ => 0,
    };

    println!("Number at that position is {}", numbers[selected_index]);
}
