use std::fs::File;
use std::io::{self, BufRead};

const PROBLEM_NUMBER: i32 = 2;

pub fn run() -> io::Result<()> {
    let file_path = "input/day2.txt";

    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut safe_line_count = 0;

    for line in reader.lines() {
        if let Ok(line) = line {
            if PROBLEM_NUMBER == 1 {
                if is_safe(&line) {
                    safe_line_count += 1;
                }
            } else if PROBLEM_NUMBER == 2 {
                if is_safe(&line) || can_be_made_safe(&line) {
                    safe_line_count += 1;
                }
            } else {
                panic!("Invalid problem number");
            }
        }
    }

    println!("{}", safe_line_count);

    Ok(())
}

fn is_safe(line: &str) -> bool {
    let numbers: Vec<i32> = line
        .split_whitespace()
        .filter_map(|num| num.parse::<i32>().ok())
        .collect();

    if numbers.len() < 2 {
        return true;
    }

    let mut is_increasing = true;
    let mut is_decreasing = true;
    let mut differences_valid = true;

    for i in 0..numbers.len() - 1 {
        let diff = (numbers[i + 1] - numbers[i]).abs();
        if diff < 1 || diff > 3 {
            differences_valid = false;
        }
        if numbers[i] >= numbers[i + 1] {
            is_increasing = false;
        }
        if numbers[i] <= numbers[i + 1] {
            is_decreasing = false;
        }
    }

    (is_increasing || is_decreasing) && differences_valid
}

fn can_be_made_safe(line: &str) -> bool {
    let numbers: Vec<i32> = line
        .split_whitespace()
        .filter_map(|num| num.parse::<i32>().ok())
        .collect();

    if numbers.len() < 3 {
        return true;
    }

    for i in 0..numbers.len() {
        let mut modified_numbers = numbers.clone();
        modified_numbers.remove(i);

        if is_safe(
            &modified_numbers
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<_>>()
                .join(" "),
        ) {
            return true;
        }
    }

    false
}
