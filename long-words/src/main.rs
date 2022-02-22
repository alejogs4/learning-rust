use std::cmp::Ordering;
use std::io::{self, BufRead, Write};

fn main() {
    const LONG_WORD_LEN: usize = 5;

    for line in io::stdin().lock().lines() {
        match line {
            Ok(entered_text) => match entered_text.len().cmp(&LONG_WORD_LEN) {
                Ordering::Greater => continue,
                _ => println!("{}", entered_text),
            },
            _ => continue,
        };
    }
}
