use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::usize;

fn parse_data(file_path: &str) -> Result<Vec<Vec<char>>> {
    let file = File::open(file_path)?;

    let map: Vec<Vec<char>> = BufReader::new(file)
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| line.chars().collect())
        .collect();

    Ok(map)
}

const DIRECTIONS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn explore_region(
    map: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
    start_row: usize,
    start_col: usize,
) -> (usize, usize) {
    let mut queue = VecDeque::new();
    queue.push_back((start_row, start_col));

    let plant_type = map[start_row][start_col];
    let mut area = 0;
    let mut perimeter = 0;

    while let Some((r, c)) = queue.pop_front() {
        if visited[r][c] {
            continue;
        }

        visited[r][c] = true;
        area += 1;

        for (direction_x, direction_y) in DIRECTIONS {
            let next_row = r as isize + direction_x;
            let next_col = c as isize + direction_y;

            if next_row < 0
                || next_row >= map.len() as isize
                || next_col < 0
                || next_col >= map[0].len() as isize
            {
                perimeter += 1;
            } else {
                let nr = next_row as usize;
                let nc = next_col as usize;
                if map[nr][nc] != plant_type {
                    perimeter += 1;
                } else if !visited[nr][nc] {
                    queue.push_back((nr, nc));
                }
            }
        }
    }

    (area, perimeter)
}

fn part_1(file_path: &str) -> Result<usize> {
    let map = parse_data(file_path)?;

    let rows = map.len();
    let cols = map[0].len();
    let mut visited = vec![vec![false; cols]; rows];
    let mut total_price = 0;

    for r in 0..rows {
        for c in 0..cols {
            if !visited[r][c] {
                let (area, perimeter) = explore_region(&map, &mut visited, r, c);
                total_price += area * perimeter;
            }
        }
    }

    Ok(total_price)
}

fn part_2(file_path: &str) -> Result<usize> {
    let map = parse_data(file_path)?;

    Ok(0)
}

fn main() -> Result<()> {
    let part_1_result = part_1("input/data")?;
    println!(" -> Part one result: {part_1_result}");

    let part_2_result = part_2("input/test_data")?;
    println!(" -> Part two result: {part_2_result}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expect_part_1_return_correct_result() {
        let part_1_result = part_1("input/test_data").expect("Should return correct value!");
        assert_eq!(part_1_result, 1930);
    }

    #[test]
    fn expect_part_2_return_correct_result() {
        let part_2_result = part_2("input/test_data").expect("Should return correct value!");
        assert_eq!(part_2_result, 0);
    }
}
