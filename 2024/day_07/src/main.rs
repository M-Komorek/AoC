use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::usize;

fn parse_data(file_path: &str) -> Result<Vec<(usize, Vec<usize>)>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let data: Vec<(usize, Vec<usize>)> = reader
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            let mut parts = line.split(':');

            let calibration_result = parts
                .next()
                .expect("should be calibration result")
                .trim()
                .parse::<usize>()
                .expect("usize value");

            let values: Vec<usize> = parts
                .next()
                .expect("should be values")
                .trim()
                .split_whitespace()
                .filter_map(|x| x.parse::<usize>().ok())
                .collect();

            (calibration_result, values)
        })
        .collect();

    Ok(data)
}

fn check_all_expressions(values: &[usize], target: usize, current_calibration: usize) -> bool {
    if values.is_empty() {
        return current_calibration == target;
    }

    let value = values[0];
    check_all_expressions(&values[1..], target, value + current_calibration)
        || check_all_expressions(&values[1..], target, value * current_calibration)
}

fn part_1(file_path: &str) -> Result<usize> {
    let data = parse_data(file_path)?;

    let calibration_sum: usize = data
        .iter()
        .filter(|(target, values)| check_all_expressions(values, *target, 0))
        .map(|(target, _)| target)
        .sum();

    Ok(calibration_sum)
}

fn combine(left: usize, right: usize) -> usize {
    let right_str = right.to_string();
    let left_str = left.to_string();
    format!("{}{}", left_str, right_str)
        .parse::<usize>()
        .unwrap()
}

fn check_all_expressions_2(values: &[usize], target: usize, current_calibration: usize) -> bool {
    if values.is_empty() {
        return current_calibration == target;
    }

    let value = values[0];
    check_all_expressions_2(&values[1..], target, value + current_calibration)
        || check_all_expressions_2(&values[1..], target, value * current_calibration)
        || check_all_expressions_2(&values[1..], target, combine(current_calibration, value))
}

fn part_2(file_path: &str) -> Result<usize> {
    let data = parse_data(file_path)?;

    let calibration_sum: usize = data
        .iter()
        .filter(|(target, values)| check_all_expressions_2(values, *target, 0))
        .map(|(target, _)| target)
        .sum();

    Ok(calibration_sum)
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
        assert_eq!(part_1_result, 3749);
    }

    #[test]
    fn expect_part_2_return_correct_result() {
        let part_2_result = part_2("input/test_data").expect("Should return correct value!");
        assert_eq!(part_2_result, 11387);
    }
}
