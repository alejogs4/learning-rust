use std::cmp::Ordering;

fn main() {
    println!("---------------- farenheit to celcius calc --------------------");
    let farenheit_temps = vec![32.4, 54.0];
    let celcius_temps = farenheit_to_celcius(farenheit_temps);
    for celcius_temp in celcius_temps {
        println!("{}", celcius_temp)
    }

    println!("------------ Fibonacci -------------------");
    println!("for 2 {}", fibo(5));

    println!("------------ Max numbers -----------------");
    let numbers = vec![32, 54];
    // println!("{}", get_max(numbers));
    println!("{}", get_max_cmp(numbers));
}

fn farenheit_to_celcius(temps: Vec<f32>) -> Vec<f32> {
    let celcius_temps: Vec<f32> = {
        let mut resulting_temps = Vec::new();
        for index in 0..temps.len() {
            let farenheit_temp = temps[index];
            resulting_temps.push((farenheit_temp - 32.0) * (5.0 / 9.0));
        }
        resulting_temps
    };
    // Using while construct
    // let mut index = 0;
    // while index <= temps.len() {
    //     let farenheit_temp = match temps.get(index) {
    //         Some(farenheit_temp) => farenheit_temp,
    //         None => &32.0,
    //     };
    //     celcius_temps.push((farenheit_temp - 32.0) * (5.0 / 9.0));
    //     index = index + 1;
    // }

    celcius_temps
}

fn fibo(n: i32) -> i32 {
    if n <= 1 {
        return 1;
    }
    fibo(n - 1) + fibo(n - 2)
}

fn get_max(nums: Vec<i32>) -> i32 {
    let max_val = nums.iter().reduce(|max_val, number| {
        if number > max_val {
            return number;
        }
        max_val
    });

    match max_val {
        Some(number) => *number,
        None => 0,
    }
}

fn get_max_cmp(nums: Vec<i32>) -> i32 {
    let mut max_val = -99999999;
    for number in nums {
        max_val = match number.cmp(&max_val) {
            Ordering::Greater => number,
            _ => max_val,
        };
    }
    max_val
}
