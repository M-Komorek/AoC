use std::cmp;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::u64;

#[allow(dead_code)]
fn read_data(file_path: &str) -> Result<(Vec<u64>, Vec<u64>)> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let data_lines_count = reader.lines().count();
    let mut left_column = Vec::with_capacity(data_lines_count);
    let mut right_column = Vec::with_capacity(data_lines_count);

    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let mut iter = line.split_whitespace();
        let left_value = iter.next().unwrap().parse::<u64>().unwrap();
        let right_value = iter.next().unwrap().parse::<u64>().unwrap();

        left_column.push(left_value);
        right_column.push(right_value);
    }

    Ok((left_column, right_column))
}

fn part_1(file_path: &str) -> Result<u64> {
    let (mut left_columnm, mut right_column) = read_data(file_path)?;
    left_columnm.sort();
    right_column.sort();

    let mut result = 0;
    for (left_value, right_value) in left_columnm.iter().zip(right_column.iter()) {
        result += cmp::max(left_value, right_value) - cmp::min(left_value, right_value);
    }

    Ok(result)
}

fn part_2(file_path: &str) -> Result<u64> {
    let (left_columnm, right_column) = read_data(file_path)?;

    let mut count_map = HashMap::new();
    for num in right_column {
        *count_map.entry(num).or_insert(0) += 1;
    }

    let mut similarity = 0;
    for num in left_columnm {
        if let Some(count) = count_map.get(&num) {
            similarity += num * count;
        }
    }

    Ok(similarity)
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
        assert_eq!(part_1_result, 11);
    }

    #[test]
    fn expect_part_2_return_correct_result() {
        let part_2_result = part_2("input/test_data").expect("Should return correct value!");
        assert_eq!(part_2_result, 31);
    }
}
