use std::fs::File;
use std::io::{Read, Result};

fn parse_data(file_path: &str) -> Result<()> {
    Ok()
}

fn part_1(file_path: &str) -> Result<usize> {
    Ok(0)
}

fn part_2(file_path: &str) -> Result<usize> {
    Ok(0)
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
        assert_eq!(part_1_result, 0);
    }

    #[test]
    fn expect_part_2_return_correct_result() {
        let part_2_result = part_2("input/test_data").expect("Should return correct value!");
        assert_eq!(part_2_result, 0);
    }
}
