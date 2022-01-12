fn main() {
    println!("Hello, world!");
    let numbers = to_strs([1, 2, 3, 4, 5, 6, 7, 8, 10]);
    print_labeled_value('v', 2);
}

fn to_strs(numbers: [i32; 9]) -> [String; 9] {
    numbers.map(|num| num.to_string())
}

fn print_labeled_value(tag: char, value: i32) {
    println!("{} = {}", tag, value);
}
