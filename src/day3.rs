use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};

const PROBLEM_NUMBER: i32 = 2;

pub fn run() -> io::Result<()> {
    match PROBLEM_NUMBER {
        1 => problem_1(),
        2 => problem_2(),
        _ => Err(io::Error::new(
            io::ErrorKind::Other,
            "Invalid problem number",
        )),
    }
}

fn problem_1() -> io::Result<()> {
    let input_file = "input/day3.txt";

    let file = File::open(input_file)?;
    let reader = io::BufReader::new(file);

    let mul_regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut total_sum = 0;

    for line in reader.lines() {
        let line = line?;

        for capture in mul_regex.captures_iter(&line) {
            let x: i32 = capture[1].parse().unwrap_or(0);
            let y: i32 = capture[2].parse().unwrap_or(0);

            total_sum += x * y;
        }
    }

    println!("{}", total_sum);

    Ok(())
}

fn problem_2() -> io::Result<()> {
    let input_file = "input/day3.txt";

    let file = File::open(input_file)?;
    let reader = io::BufReader::new(file);

    let instruction_regex = Regex::new(r"(do\(\)|don't\(\)|mul\(\d+,\d+\))").unwrap();
    let mul_regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut mul_enabled = true;

    let mut total_sum = 0;

    for line in reader.lines() {
        let line = line?;

        for capture in instruction_regex.find_iter(&line) {
            let instruction = capture.as_str();

            if instruction == "do()" {
                mul_enabled = true;
            } else if instruction == "don't()" {
                mul_enabled = false;
            } else if mul_enabled {
                if let Some(mul_match) = mul_regex.captures(instruction) {
                    let x: i32 = mul_match[1].parse().unwrap_or(0);
                    let y: i32 = mul_match[2].parse().unwrap_or(0);

                    total_sum += x * y;
                }
            }
        }
    }
    println!("{}", total_sum);

    Ok(())
}
