use std::collections::HashMap;
use std::fs::File;
use std::i32;
use std::io::{self, BufRead};
use std::path::Path;

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
    let path = "input/day1.txt";

    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    let mut dist_sum = 0;

    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(l) = line {
                let parts: Vec<&str> = l.split_whitespace().collect();
                if parts.len() == 2 {
                    if let (Ok(left_num), Ok(right_num)) =
                        (parts[0].parse::<i32>(), parts[1].parse::<i32>())
                    {
                        left.push(left_num);
                        right.push(right_num);
                    } else {
                        eprintln!("Error parsing numbers on line: {}", l);
                    }
                } else {
                    eprintln!("Invalid format on line: {}", l);
                }
            }
        }
    }

    left = merge_sort(left);
    right = merge_sort(right);

    for i in 0..left.len() {
        dist_sum += i32::abs(left[i] - right[i])
    }

    println!("{}", dist_sum);

    Ok(())
}

fn problem_2() -> io::Result<()> {
    let path = "input.txt";

    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(l) = line {
                let parts: Vec<&str> = l.split_whitespace().collect();
                if parts.len() == 2 {
                    if let (Ok(left_num), Ok(right_num)) =
                        (parts[0].parse::<i32>(), parts[1].parse::<i32>())
                    {
                        left.push(left_num);
                        right.push(right_num);
                    } else {
                        eprintln!("Error parsing numbers on line: {}", l);
                    }
                } else {
                    eprintln!("Invalid format on line: {}", l);
                }
            }
        }
    }

    left = merge_sort(left);
    right = merge_sort(right);

    let frequency_map = count_frequencies(&right);

    let sum = calculate_similarity_score(&left, &frequency_map);

    println!("{}", sum);

    Ok(())
}

fn merge_sort(mut array: Vec<i32>) -> Vec<i32> {
    if array.len() <= 1 {
        return array;
    }

    let mid = array.len() / 2;
    let left = merge_sort(array.drain(..mid).collect());
    let right = merge_sort(array);

    merge(left, right)
}

fn merge(left: Vec<i32>, right: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();
    let mut left_iter = left.into_iter();
    let mut right_iter = right.into_iter();

    let mut left_val = left_iter.next();
    let mut right_val = right_iter.next();

    while let (Some(l), Some(r)) = (left_val, right_val) {
        if l <= r {
            result.push(l);
            left_val = left_iter.next();
        } else {
            result.push(r);
            right_val = right_iter.next();
        }
    }

    result.extend(left_val);
    result.extend(left_iter);
    result.extend(right_val);
    result.extend(right_iter);

    result
}

fn count_frequencies(numbers: &[i32]) -> HashMap<i32, i32> {
    let mut frequency_map = HashMap::new();
    for &num in numbers {
        *frequency_map.entry(num).or_insert(0) += 1;
    }
    frequency_map
}

fn calculate_similarity_score(numbers: &[i32], frequency_map: &HashMap<i32, i32>) -> i32 {
    numbers
        .iter()
        .map(|&num| num * frequency_map.get(&num).unwrap_or(&0))
        .sum()
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
