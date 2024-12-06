use std::collections::HashSet;
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader, Error, ErrorKind, Result};
use std::{isize, usize};

//
//
// This code is a totally mess but it works.
//
// No time to make it good.
//
//

fn parse_data(file_path: &str) -> Result<Vec<Vec<char>>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let grid: Vec<Vec<char>> = reader
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| line.chars().collect())
        .collect();

    Ok(grid)
}

fn find_guard_position(grid: &Vec<Vec<char>>) -> Result<(isize, isize)> {
    for guard_y in 0..grid.len() {
        if let Some(guard_x) = grid[guard_y].iter().position(|mark| *mark == '^') {
            return Ok((guard_x as isize, guard_y as isize));
        }
    }

    Err(Error::new(ErrorKind::Other, "Guard not found"))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_right(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn step(self, pos_x: isize, pos_y: isize) -> (isize, isize) {
        match self {
            Direction::Up => (pos_x, pos_y - 1),
            Direction::Right => (pos_x + 1, pos_y),
            Direction::Down => (pos_x, pos_y + 1),
            Direction::Left => (pos_x - 1, pos_y),
        }
    }
}

fn part_1(file_path: &str) -> Result<u64> {
    let grid = parse_data(file_path)?;
    let height_limit = 0..grid.len() as isize;
    let width_limit = 0..grid[0].len() as isize;

    let (mut guard_x, mut guard_y) = find_guard_position(&grid)?;
    let mut guard_dir = Direction::Up;

    let mut visited = HashSet::new();

    while width_limit.contains(&guard_x) && height_limit.contains(&guard_y) {
        visited.insert((guard_x, guard_y));

        let (next_guard_x, next_guard_y) = guard_dir.step(guard_x, guard_y);

        if width_limit.contains(&next_guard_x)
            && height_limit.contains(&next_guard_y)
            && grid[next_guard_y as usize][next_guard_x as usize] == '#'
        {
            guard_dir = guard_dir.turn_right();
        } else {
            guard_x = next_guard_x;
            guard_y = next_guard_y;
        }
    }

    Ok(visited.len() as u64)
}

fn simulate_guard_with_obstruction(
    grid: &[Vec<char>],
    guard_start_x: isize,
    guard_start_y: isize,
    obstruction: Option<(isize, isize)>,
) -> bool {
    let mut visited_states = HashSet::new();

    let mut guard_dir = Direction::Up;
    let mut guard_x = guard_start_x;
    let mut guard_y = guard_start_y;

    let height_limit = 0..grid.len() as isize;
    let width_limit = 0..grid[0].len() as isize;

    loop {
        if !width_limit.contains(&guard_x) || !height_limit.contains(&guard_y) {
            return false;
        }

        let state = (guard_x, guard_y, guard_dir);
        if visited_states.contains(&state) {
            return true; // A loop is detected
        }
        visited_states.insert(state);

        let (next_guard_x, next_guard_y) = guard_dir.step(guard_x, guard_y);
        let is_obstruction = Some((next_guard_x, next_guard_y)) == obstruction;

        if width_limit.contains(&next_guard_x)
            && height_limit.contains(&next_guard_y)
            && (grid[next_guard_y as usize][next_guard_x as usize] == '#' || is_obstruction)
        {
            guard_dir = guard_dir.turn_right(); // Turn right
        } else {
            guard_x = next_guard_x;
            guard_y = next_guard_y; // Move forward
        }
    }
}

fn part_2(file_path: &str) -> Result<u64> {
    let grid = parse_data(file_path)?;
    let (guard_start_x, guard_start_y) = find_guard_position(&grid)?;

    let mut visited_positions = HashSet::new();
    let mut guard_dir = Direction::Up;
    let mut guard_x = guard_start_x;
    let mut guard_y = guard_start_y;

    let height_limit = 0..grid.len() as isize;
    let width_limit = 0..grid[0].len() as isize;

    loop {
        if !width_limit.contains(&guard_x) || !height_limit.contains(&guard_y) {
            break;
        }

        visited_positions.insert((guard_x, guard_y));

        let (next_guard_x, next_guard_y) = guard_dir.step(guard_x, guard_y);
        if width_limit.contains(&next_guard_x)
            && height_limit.contains(&next_guard_y)
            && grid[next_guard_y as usize][next_guard_x as usize] == '#'
        {
            guard_dir = guard_dir.turn_right();
        } else {
            guard_x = next_guard_x;
            guard_y = next_guard_y;
        }
    }

    let mut possible_obstructions = 0;
    for &(x, y) in &visited_positions {
        if x == guard_start_x && y == guard_start_y {
            continue;
        }

        if simulate_guard_with_obstruction(&grid, guard_start_x, guard_start_y, Some((x, y))) {
            possible_obstructions += 1;
        }
    }

    Ok(possible_obstructions)
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
        let part_1_result = part_1("input/test_data").expect("Should return correct value!");
        assert_eq!(part_1_result, 41);
    }

    #[test]
    fn expect_part_2_return_correct_result() {
        let part_2_result = part_2("input/test_data").expect("Should return correct value!");
        assert_eq!(part_2_result, 6);
    }
}
