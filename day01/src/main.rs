use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    sum().unwrap()
}

pub fn sum() -> Result<(), std::io::Error> {
    let file_path =
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("input.txt");

    println!("{}", file_path.display());

    let file: File = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut sum_1: u32 = 0;
    let mut sum_2: u32 = 0;

    for line in reader.lines() {
        let line = line?;

        match find_digits_1(&line) {
            Ok(result) => {
                println!("1: {} -> {}", line, result);
                sum_1 += result
            },
            Err(err) => println!("Error: {}", err)
        }

        match find_digits_2(&line) {
            Ok(result) => {
                println!("2: {} -> {}", line, result);
                sum_2 += result
            },
            Err(err) => println!("Error: {}", err)
        }
    }

    println!("1: {}", sum_1);
    println!("2: {}", sum_2);

    return Ok(())
}

fn find_digits_1(line: &str) -> Result<u32, &'static str> {
    let mut left = line.bytes();
    let mut right = line.bytes().rev();

    let left_digit_char = left.find(|&b| b.is_ascii_digit());
    let right_digit_char = right.find(|&b| b.is_ascii_digit());

    match (left_digit_char, right_digit_char) {
        (Some(left_digit_char), Some(right_digit_char)) => {
            let combined = format!(
                "{}{}",
                left_digit_char as char,
                right_digit_char as char);

            combined.parse::<u32>().map_err(|_| "Parsing error")
        },
        _ => Err("Both characters must be digits"),
    }
}

fn find_digits_2(line: &str) -> Result<u32, &'static str> {
    let mut tracker = DigitTracker::new();
    let mut char_indices = line.char_indices();
    let length = line.len();

    while let Some((i, c)) = char_indices.next() {
        if c.is_ascii_digit() {
            tracker.set(c);
        } else {
            if c == 'o' {
                if i + 3 <= length && &line[i..i + 3] == "one" {
                    tracker.set('1');
                }
            }

            if c == 't' {
                if i + 3 <= length && &line[i..i + 3] == "two" {
                    tracker.set('2');
                }

                if i + 5 <= length && &line[i..i + 5] == "three" {
                    tracker.set('3');
                    char_indices.nth(2);
                }
            }

            if c == 'f' {
                if i + 4 <= length && &line[i..i + 4] == "four" {
                    tracker.set('4');
                    char_indices.nth(1);
                }

                if i + 4 <= length && &line[i..i + 4] == "five" {
                    tracker.set('5');
                    char_indices.nth(1);
                }
            }

            if c == 's' {
                if i + 3 <= length && &line[i..i + 3] == "six" {
                    tracker.set('6');
                }

                if i + 5 <= length && &line[i..i + 5] == "seven" {
                    tracker.set('7');
                    char_indices.nth(2);
                }
            }

            if c == 'e' {
                if i + 5 <= length && &line[i..i + 5] == "eight" {
                    tracker.set('8');
                    char_indices.nth(2);
                }
            }

            if c == 'n' {
                if i + 4 <= length && &line[i..i + 4] == "nine" {
                    tracker.set('9');
                    char_indices.nth(1);
                }
            }
        }
    }

    let combined = format!(
        "{}{}",
        tracker.first_digit.unwrap(),
        tracker.last_digit.unwrap());

    combined.parse::<u32>().map_err(|_| "Parsing error")
}

struct DigitTracker {
    first_digit: Option<char>,
    last_digit: Option<char>
}

impl DigitTracker {
    fn new() -> Self {
        Self {
            first_digit: None,
            last_digit: None
        }
    }

    fn set(&mut self, digit: char) {
        if self.first_digit.is_none() {
            self.first_digit = Some(digit)
        }

        self.last_digit = Some(digit)
    }
}