use std::collections::BinaryHeap;
use std::io::BufRead;

use std::fs;
use std::io;

fn read_data(file_path: &str) -> io::Result<io::BufReader<fs::File>> {
    let file = fs::File::open(file_path)?;
    let reader = io::BufReader::new(file);
    Ok(reader)
}

fn parse_to_elves_calories(data_reader: io::BufReader<fs::File>) -> io::Result<BinaryHeap<u32>> {
    let mut elve_calories: u32 = 0;
    let mut elves_calories = BinaryHeap::new();

    for line in data_reader.lines() {
        let line = line?;

        if line.is_empty() {
            elves_calories.push(elve_calories);
            elve_calories = 0;
        } else {
            elve_calories += line
                .parse::<u32>()
                .expect("Should be parsed to u32!")
        }
    }

    if elve_calories != 0 {
        elves_calories.push(elve_calories);
    }

    Ok(elves_calories)
}

fn part_1(file_path: &str) -> io::Result<u32> {
    let reader = read_data(file_path)?;
    let mut elves_calories = parse_to_elves_calories(reader)?;

    Ok(elves_calories.pop().expect("Should exist!"))
}

fn part_2(file_path: &str) -> io::Result<u32> {
    let reader = read_data(file_path)?;
    let mut elves_calories = parse_to_elves_calories(reader)?;

    let mut max_three: u32 = 0;
    for _ in 0..3 {
        max_three += elves_calories.pop().expect("Should exist!");
    }

    Ok(max_three)
}

fn main() -> io::Result<()> {
    let part_1_result = part_1("data/data.txt")?;
    println!("{part_1_result}");

    let part_2_result = part_2("data/data.txt")?;
    println!("{part_2_result}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expect_part_1_return_correct_result() {
        let part_1_result = part_1("data/test_data.txt").expect("Should return correct value!");
        assert_eq!(part_1_result, 24000);
    }

    #[test]
    fn expect_part_2_return_correct_result() {
        let part_2_result = part_2("data/test_data.txt").expect("Should return correct value!");
        assert_eq!(part_2_result, 45000);
    }
}
