use std::{cmp::Ordering, ops::Div};

struct Student {
    name: String,
    score: f32,
}

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

    println!("------------ Get students with good score -----------------");
    let all_students: Vec<Student> = vec![
        Student {
            name: String::from("Alejandro"),
            score: 3.0,
        },
        Student {
            name: String::from("Miguel"),
            score: 4.2,
        },
        Student {
            name: String::from("Laura"),
            score: 2.3,
        },
    ];
    // let approved_students = get_approved_students(all_students, 3.0);
    // for student in approved_students {
    //     println!("{} with {}", student.name, student.score);
    // }

    println!("------------ Better score than -----------------");
    println!("{}", is_average_score_better_than(all_students, 7.0));
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

fn get_approved_students(students: Vec<Student>, passing_note: f32) -> Vec<Student> {
    // Option 1 (long one)
    // let mut approved_students: Vec<Student> = Vec::new();
    // for student in students {
    //     if student.score >= passing_note {
    //         approved_students.push(student);
    //     }
    // }
    // approved_students

    // Option 2
    let approved_students: Vec<Student> = students
        .into_iter() // This iterates over values rather than references
        .filter(|student| student.score >= passing_note)
        .collect();
    approved_students
}

fn is_average_score_better_than(students: Vec<Student>, target_score: f32) -> bool {
    // Option 1
    let mut accumulated_score: f32 = 0.0;
    for student in &students {
        accumulated_score = accumulated_score + student.score;
    }
    let average_score = accumulated_score.div(students.len() as f32);
    let is_better = average_score >= target_score;
    is_better

    // Option 2
    // let accumulated_score = students
    //     .iter()
    //     .map(|student| student.score)
    //     .reduce(|accumulated, score| accumulated + score);

    // match accumulated_score {
    //     Some(score) => (score.div(students.len() as f32)) >= target_score,
    //     None => false,
    // }
}
