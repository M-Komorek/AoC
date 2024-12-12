use std::collections::HashMap;
use std::io::Result;

fn parse_data(file_path: &str) -> Result<HashMap<usize, usize>> {
    let data = std::fs::read_to_string(file_path)?;
    let mut stones = HashMap::new();

    for stone in data.split_whitespace().map(|s| s.parse::<usize>().unwrap()) {
        *stones.entry(stone).or_insert(0) += 1;
    }

    Ok(stones)
}

fn split_digits(num: usize) -> (usize, usize) {
    let num_str = num.to_string();
    let len = num_str.len();
    let mid = len / 2;

    let left_str = &num_str[..mid];
    let right_str = &num_str[mid..];

    let left = left_str.parse::<usize>().unwrap_or(0);
    let right = right_str.parse::<usize>().unwrap_or(0);

    (left, right)
}

fn process_stones(mut stones: HashMap<usize, usize>, blinks: usize) -> usize {
    for blink in 0..blinks {
        dbg!(blink);

        let mut new_stones = HashMap::new();

        for (&stone, &count) in stones.iter() {
            if stone == 0 {
                *new_stones.entry(1).or_insert(0) += count;
            } else if stone.to_string().len() % 2 == 0 {
                let (left, right) = split_digits(stone);
                *new_stones.entry(left).or_insert(0) += count;
                *new_stones.entry(right).or_insert(0) += count;
            } else {
                *new_stones.entry(stone * 2024).or_insert(0) += count;
            }
        }

        stones = new_stones;
    }

    stones.values().sum()
}

fn part_1(file_path: &str) -> Result<usize> {
    let stones = parse_data(file_path)?;
    let final_stones_count = process_stones(stones, 25);

    Ok(final_stones_count)
}

fn part_2(file_path: &str) -> Result<usize> {
    let stones = parse_data(file_path)?;
    let final_stones_count = process_stones(stones, 75);

    Ok(final_stones_count)
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
        assert_eq!(part_1_result, 55312);
    }

    #[test]
    fn expect_part_2_return_correct_result() {
        let part_2_result = part_2("input/test_data").expect("Should return correct value!");
        assert_eq!(part_2_result, 55312);
    }
}
