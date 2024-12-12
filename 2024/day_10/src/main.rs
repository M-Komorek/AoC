use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn parse_data(file_path: &str) -> Result<Vec<Vec<u8>>> {
    let mut map = Vec::new();
    let file = File::open(file_path)?;

    for line in BufReader::new(file).lines() {
        let row: Vec<u8> = line?
            .chars()
            .filter_map(|ch| ch.to_digit(10).map(|d| d as u8))
            .collect();
        map.push(row);
    }

    Ok(map)
}

fn find_trailheads(map: &[Vec<u8>]) -> Vec<(usize, usize)> {
    let mut trailheads = Vec::new();

    for (y, row) in map.iter().enumerate() {
        for (x, &value) in row.iter().enumerate() {
            if value == 0 {
                trailheads.push((x, y));
            }
        }
    }

    trailheads
}

fn calculate_trailhead_score(map: &[Vec<u8>], start_x: usize, start_y: usize) -> usize {
    let mut visited = vec![vec![false; map[0].len()]; map.len()];
    let mut reachable_9s = 0;

    let mut stack = vec![(start_x, start_y, 0)];
    while let Some((x, y, height)) = stack.pop() {
        if visited[y][x] {
            continue;
        }
        visited[y][x] = true;

        if map[y][x] == 9 {
            reachable_9s += 1;
            continue;
        }

        let neighbors = [
            (x.wrapping_sub(1), y), // left
            (x + 1, y),             // right
            (x, y.wrapping_sub(1)), // up
            (x, y + 1),             // down
        ];

        for &(next_x, next_y) in &neighbors {
            if next_x < map[0].len()
                && next_y < map.len()
                && !visited[next_y][next_x]
                && map[next_y][next_x] == height + 1
            {
                stack.push((next_x, next_y, height + 1));
            }
        }
    }

    reachable_9s
}

fn part_1(file_path: &str) -> Result<usize> {
    let map = parse_data(file_path)?;
    let trailheads = find_trailheads(&map);

    let total_score: usize = trailheads
        .iter()
        .map(|&(x, y)| calculate_trailhead_score(&map, x, y))
        .sum();

    Ok(total_score)
}

fn count_paths(map: &[Vec<u8>], x: usize, y: usize, height: u8) -> usize {
    // Base case
    if map[y][x] != height {
        return 0;
    }

    // Base case
    if height == 9 {
        return 1;
    }

    let neighbors = [
        (x.wrapping_sub(1), y), // left
        (x + 1, y),             // right
        (x, y.wrapping_sub(1)), // up
        (x, y + 1),             // down
    ];

    let mut total_paths = 0;
    for &(nx, ny) in &neighbors {
        if nx < map[0].len() && ny < map.len() {
            total_paths += count_paths(map, nx, ny, height + 1);
        }
    }

    total_paths
}

fn part_2(file_path: &str) -> Result<usize> {
    let map = parse_data(file_path)?;
    let trailheads = find_trailheads(&map);

    let total_rating: usize = trailheads
        .iter()
        .map(|&(x, y)| count_paths(&map, x, y, 0))
        .sum();

    Ok(total_rating)
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
        assert_eq!(part_1_result, 36);
    }

    #[test]
    fn expect_part_2_return_correct_result() {
        let part_2_result = part_2("input/test_data").expect("Should return correct value!");
        assert_eq!(part_2_result, 81);
    }
}
