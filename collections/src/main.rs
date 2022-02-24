use std::collections::HashMap;

fn main() {
    // ------------ Vectors ----------
    let mut nums: Vec<u32> = Vec::new();
    // Adding
    nums.push(1);
    nums.push(2);
    nums.push(3);
    // Updating
    nums[0] = 100;
    // Reading
    match nums.get(0) {
        Some(num) => println!("{}", num),
        None => println!("Nothing"),
    };
    let num_ref = &nums[0];
    // iterating
    for num in &nums {
        println!("{} {}", num, num_ref)
    }

    // --------- Strings ------
    // Create
    let mut str = String::from("Hello");
    str.push_str(", World");
    str.push('.');
    println!("{}", str);

    // concatenate
    let concatenated_str = str + " Concat";
    println!("{}", concatenated_str);

    let concat_no_ownership = format!("{}{}", concatenated_str, " More");
    println!("{} {}", concatenated_str, concat_no_ownership);

    // Access
    let first_two = &concat_no_ownership[0..2];
    println!("{}", first_two);

    // Iterate
    for chunk in concat_no_ownership.chars() {
        print!("{}", chunk);
    }
    println!("");

    // --------- HashMap ------
    let mut people = HashMap::new();
    people.insert("Alejandro", 22);
    people.insert("Miguel", 22);
    people.insert("Mauro", 24);
    people.entry("Laura").or_insert(21);

    // Update
    people.insert("Laura", 22);

    // Update based on previous value
    let age = people.get_mut("Alejandro").unwrap();
    *age += 1;
    // Iterate
    for (key, val) in people {
        println!("{} = {}", key, val);
    }
}
