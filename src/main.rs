use std::io;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

const DAY_NUMBER: i32 = 5;

fn main() -> io::Result<()> {
    match DAY_NUMBER {
        1 => day1::run(),
        2 => day2::run(),
        3 => day3::run(),
        4 => day4::run(),
        5 => day5::run(),
        _ => Err(io::Error::new(io::ErrorKind::Other, "Invalid day number")),
    }
}
