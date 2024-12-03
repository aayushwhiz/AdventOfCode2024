use std::io;
mod day1;
mod day2;
mod day3;

const DAY_NUMBER: i32 = 3;

fn main() -> io::Result<()> {
    match DAY_NUMBER {
        1 => day1::run(),
        2 => day2::run(),
        3 => day3::run(),
        _ => Err(io::Error::new(io::ErrorKind::Other, "Invalid day number")),
    }
}
