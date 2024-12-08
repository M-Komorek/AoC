use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::usize;

#[derive(Clone, Debug)]
struct Antenna {
    x: usize,
    y: usize,
}

fn read_data(file_path: &str) -> Result<Vec<Vec<char>>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let grid: Vec<Vec<char>> = reader
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| line.chars().collect())
        .collect();

    Ok(grid)
}

fn parse_data(data: &[Vec<char>]) -> Result<HashMap<char, Vec<Antenna>>> {
    let mut antennas_map: HashMap<char, Vec<Antenna>> = HashMap::new();
    for (y, line) in data.iter().enumerate() {
        for (x, freq) in line.iter().enumerate() {
            if *freq != '.' {
                let antenna = Antenna { x, y };
                antennas_map
                    .entry(*freq)
                    .or_insert_with(Vec::new)
                    .push(antenna);
            }
        }
    }

    Ok(antennas_map)
}

fn part_1(file_path: &str) -> Result<usize> {
    let data = read_data(file_path)?;
    let antennas_map = parse_data(&data)?;

    let mut unique_antinode = HashSet::new();
    let height_limit = 0..data.len() as isize;
    let width_limit = 0..data[0].len() as isize;

    for (_, antennas) in antennas_map {
        let mut unique_pairs = Vec::new();
        for i in 0..antennas.len() {
            for j in (i + 1)..antennas.len() {
                unique_pairs.push((antennas[i].clone(), antennas[j].clone()));
            }
        }

        for (antenna_l, antenna_r) in unique_pairs {
            let dx = antenna_r.x as isize - antenna_l.x as isize;
            let dy = antenna_r.y as isize - antenna_l.y as isize;

            let extended_x1 = antenna_l.x as isize - dx;
            let extended_y1 = antenna_l.y as isize - dy;
            if width_limit.contains(&extended_x1) && height_limit.contains(&extended_y1) {
                unique_antinode.insert((extended_x1 as usize, extended_y1 as usize));
            }

            let extended_x2 = antenna_r.x as isize + dx;
            let extended_y2 = antenna_r.y as isize + dy;
            if width_limit.contains(&extended_x2) && height_limit.contains(&extended_y2) {
                unique_antinode.insert((extended_x2 as usize, extended_y2 as usize));
            }
        }
    }

    Ok(unique_antinode.len())
}

fn part_2(file_path: &str) -> Result<usize> {
    let data = read_data(file_path)?;
    let antennas_map = parse_data(&data)?;

    let mut unique_antinode = HashSet::new();
    let height_limit = 0..data.len() as isize;
    let width_limit = 0..data[0].len() as isize;

    for (_, antennas) in antennas_map {
        let mut unique_pairs = Vec::new();
        for i in 0..antennas.len() {
            for j in (i + 1)..antennas.len() {
                unique_pairs.push((antennas[i].clone(), antennas[j].clone()));
            }
        }

        for (antenna_l, antenna_r) in unique_pairs {
            let dx = antenna_r.x as isize - antenna_l.x as isize;
            let dy = antenna_r.y as isize - antenna_l.y as isize;

            let mut extended_x1 = antenna_l.x as isize - dx;
            let mut extended_y1 = antenna_l.y as isize - dy;
            while width_limit.contains(&extended_x1) && height_limit.contains(&extended_y1) {
                unique_antinode.insert((extended_x1 as usize, extended_y1 as usize));
                extended_x1 -= dx;
                extended_y1 -= dy;
            }

            let mut extended_x2 = antenna_r.x as isize + dx;
            let mut extended_y2 = antenna_r.y as isize + dy;
            while width_limit.contains(&extended_x2) && height_limit.contains(&extended_y2) {
                unique_antinode.insert((extended_x2 as usize, extended_y2 as usize));
                extended_x2 += dx;
                extended_y2 += dy;
            }
        }

        for antenna in antennas {
            unique_antinode.insert((antenna.x, antenna.y));
        }
    }

    Ok(unique_antinode.len())
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
        assert_eq!(part_1_result, 14);
    }

    #[test]
    fn expect_part_2_return_correct_result() {
        let part_2_result = part_2("input/test_data").expect("Should return correct value!");
        assert_eq!(part_2_result, 34);
    }
}
