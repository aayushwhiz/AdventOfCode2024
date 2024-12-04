use std::io;

const PROBLEM_NUMBER: i32 = 1;

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
    Ok(())
}

fn problem_2() -> io::Result<()> {
    Ok(())
}
