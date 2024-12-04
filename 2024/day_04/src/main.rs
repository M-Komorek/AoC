use std::fs::File;
use std::io::{BufRead, BufReader, Read, Result};
use std::{isize, usize};

#[allow(dead_code)]
fn parse_data(file_path: &str) -> Result<Vec<Vec<char>>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let data: Vec<Vec<char>> = reader
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| line.chars().collect())
        .collect();

    Ok(data)
}

const DIRECTIONS: [(isize, isize); 8] = [
    (0, 1),   // Right
    (1, 0),   // Down
    (1, 1),   // Down-Right
    (1, -1),  // Down-Left
    (0, -1),  // Left
    (-1, 0),  // Up
    (-1, -1), // Up-Left
    (-1, 1),  // Up-Right
];

fn is_word_at(
    grid: &Vec<Vec<char>>,
    word: &[char],
    start_row: usize,
    start_col: usize,
    direction_x: isize,
    direction_y: isize,
) -> bool {
    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;

    for (i, &mark) in word.iter().enumerate() {
        let row = start_row as isize + i as isize * direction_x;
        let col = start_col as isize + i as isize * direction_y;

        if row < 0 || row >= rows {
            return false;
        }

        if col < 0 || col >= cols {
            return false;
        }

        if grid[row as usize][col as usize] != mark {
            return false;
        }
    }

    true
}

fn part_1(file_path: &str) -> Result<u64> {
    let data = parse_data(file_path).unwrap();
    let word = ['X', 'M', 'A', 'S'];

    let mut res = 0;

    for (i, row) in data.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            for (direction_x, direction_y) in DIRECTIONS {
                if is_word_at(&data, &word, i, j, direction_x, direction_y) {
                    res += 1;
                }
            }
        }
    }

    Ok(res)
}

fn is_x_mas_at(grid: &Vec<Vec<char>>, start_row: usize, start_col: usize) -> bool {
    (grid[start_row - 1][start_col + 1] == 'M' && grid[start_row + 1][start_col - 1] == 'S'
        || grid[start_row - 1][start_col + 1] == 'S' && grid[start_row + 1][start_col - 1] == 'M')
        && (grid[start_row + 1][start_col + 1] == 'M' && grid[start_row - 1][start_col - 1] == 'S'
            || grid[start_row + 1][start_col + 1] == 'S'
                && grid[start_row - 1][start_col - 1] == 'M')
}

fn part_2(file_path: &str) -> Result<u64> {
    let data = parse_data(file_path).unwrap();

    let mut res = 0;

    for i in 1..data.len() - 1 {
        for j in 1..data[0].len() - 1 {
            if data[i][j] == 'A' {
                if is_x_mas_at(&data, i, j) {
                    res += 1;
                }
            }
        }
    }

    Ok(res)
}

fn main() -> Result<()> {
    let part_1_result = part_1("input/data")?;
    println!(" -> Part one result: {part_1_result}");

    let part_2_result = part_2("input/data")?;
    println!(" -> Part two result: {part_2_result}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expect_part_1_return_correct_result() {
        let part_1_result = part_1("input/test_data_1").expect("Should return correct value!");
        assert_eq!(part_1_result, 18);
    }

    #[test]
    fn expect_part_2_return_correct_result() {
        let part_2_result = part_2("input/test_data_2").expect("Should return correct value!");
        assert_eq!(part_2_result, 9);
    }
}
