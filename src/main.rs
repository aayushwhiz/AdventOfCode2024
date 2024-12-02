use std::io;
mod day1;
mod day2;

const DAY_NUMBER: i32 = 2;

fn main() -> io::Result<()> {
    match DAY_NUMBER {
        1 => day1::run(),
        2 => day2::run(),
        _ => Err(io::Error::new(io::ErrorKind::Other, "Invalid day number")),
    }
}
