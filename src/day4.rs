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
    let file_path = "input/day4.txt";
    let grid = read_grid_from_file(file_path)?;

    let result = count_xmas(&grid);
    println!("Number of times XMAS appears: {}", result);
    Ok(())
}

fn problem_2() -> io::Result<()> {
    let file_path = "input/day4.txt";
    let grid = read_grid_from_file(file_path)?;

    let result = count_x_mas(&grid);
    println!("Number of X-MAS patterns found: {}", result);
    Ok(())
}

fn count_xmas(grid: &[Vec<char>]) -> usize {
    let directions = [
        (-1, -1), // Up-left diagonal
        (-1, 0),  // Up
        (-1, 1),  // Up-right diagonal
        (0, -1),  // Left
        (0, 1),   // Right
        (1, -1),  // Down-left diagonal
        (1, 0),   // Down
        (1, 1),   // Down-right diagonal
    ];

    let word = "XMAS";
    let rows = grid.len();
    let cols = if rows > 0 { grid[0].len() } else { 0 };

    let mut count = 0;

    for row in 0..rows {
        for col in 0..cols {
            for dir in directions.iter() {
                let mut matched = true;
                for (i, c) in word.chars().enumerate() {
                    let new_row = row as isize + dir.0 * i as isize;
                    let new_col = col as isize + dir.1 * i as isize;

                    if new_row < 0
                        || new_col < 0
                        || new_row >= rows as isize
                        || new_col >= cols as isize
                        || grid[new_row as usize][new_col as usize] != c
                    {
                        matched = false;
                        break;
                    }
                }
                if matched {
                    count += 1;
                }
            }
        }
    }

    count
}

fn count_x_mas(grid: &[Vec<char>]) -> usize {
    let mut count = 0;
    let rows = grid.len();
    let cols = if rows > 0 { grid[0].len() } else { 0 };

    for row in 1..rows - 1 {
        for col in 1..cols - 1 {
            let center = grid[row][col];

            if center == 'A'
                && (check_diagonal(grid, row, col, "M.S", "M.S")
                    || check_diagonal(grid, row, col, "S.M", "S.M")
                    || check_diagonal(grid, row, col, "M.S", "S.M")
                    || check_diagonal(grid, row, col, "S.M", "M.S"))
            {
                count += 1;
            }
        }
    }

    count
}

fn check_diagonal(
    grid: &[Vec<char>],
    row: usize,
    col: usize,
    top_left_bottom_right: &str,
    top_right_bottom_left: &str,
) -> bool {
    let (rows, cols) = (grid.len(), grid[0].len());
    let chars_tl_br: Vec<char> = top_left_bottom_right.chars().collect();
    let chars_tr_bl: Vec<char> = top_right_bottom_left.chars().collect();

    let top_left_valid = row > 0 && col > 0 && grid[row - 1][col - 1] == chars_tl_br[0];
    let bottom_right_valid =
        row + 1 < rows && col + 1 < cols && grid[row + 1][col + 1] == chars_tl_br[2];

    let top_right_valid = row > 0 && col + 1 < cols && grid[row - 1][col + 1] == chars_tr_bl[0];
    let bottom_left_valid = row + 1 < rows && col > 0 && grid[row + 1][col - 1] == chars_tr_bl[2];

    top_left_valid && bottom_right_valid && top_right_valid && bottom_left_valid
}

fn read_grid_from_file(file_path: &str) -> io::Result<Vec<Vec<char>>> {
    let mut grid = Vec::new();
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        grid.push(line.chars().collect());
    }

    Ok(grid)
}
