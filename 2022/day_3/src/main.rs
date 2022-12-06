use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::Read;

use std::fs;
use std::io;

#[allow(dead_code)]
fn create_buf_reader(file_path: &str) -> io::Result<io::BufReader<fs::File>> {
    let file = fs::File::open(file_path)?;
    let reader = io::BufReader::new(file);
    Ok(reader)
}

#[allow(dead_code)]
fn read_data_as_bytes(file_path: &str) -> io::Result<Vec<u8>> {
    let mut data_file = File::open(file_path)?;
    let mut buffer = Vec::new();

    data_file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

fn char_to_int(mark: char) -> u32 {
    match mark {
        mark @ 'A'..='Z' => mark as u32 - 38,
        mark @ 'a'..='z' => mark as u32 - 96,
        _ => unreachable!(),
    }
}

fn part_1(file_path: &str) -> io::Result<u32> {
    let reader = create_buf_reader(file_path)?;

    let sum_of_the_priorities: u32 = reader
        .lines()
        .map(|line| {
            let line = line.expect("Should contain data!");

            let (left_part, right_part) = line.split_at(line.len() / 2);
            let left_part_items: HashSet<char> = left_part.chars().collect();
            let right_part_items: HashSet<char> = right_part.chars().collect();

            let mut common_item = left_part_items.intersection(&right_part_items);
            let common_item = common_item
                .next()
                .expect("Should contain common element!")
                .clone();

            char_to_int(common_item)
        })
        .sum();

    Ok(sum_of_the_priorities)
}

// not proud of this solution but had finite time :(
fn part_2(file_path: &str) -> io::Result<u32> {
    let mut reader = create_buf_reader(file_path)?;

    let mut data_string = String::new();
    reader.read_to_string(&mut data_string)?;

    let first_backpacks = data_string.lines().clone().step_by(3);
    let seconds_backpacks = data_string.lines().clone().skip(1).step_by(3);
    let third_backpacks = data_string.lines().clone().skip(2).step_by(3);

    let sum: u32 = first_backpacks
        .zip(seconds_backpacks.zip(third_backpacks))
        .map(|(first, (second, third))| {
            let first_items: HashSet<char> = first.chars().collect();
            let second_items: HashSet<char> = second.chars().collect();
            let third: HashSet<char> = third.chars().collect();

            let common_first_second: HashSet<char> =
                first_items.intersection(&second_items).copied().collect();
            let mut common_item = common_first_second.intersection(&third);
            let common_item = common_item
                .next()
                .expect("Should contain common element!")
                .clone();

            char_to_int(common_item)
        })
        .sum();

    Ok(sum)
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
        assert_eq!(part_1_result, 157);
    }

    #[test]
    fn expect_part_2_return_correct_result() {
        let part_2_result = part_2("data/test_data.txt").expect("Should return correct value!");
        assert_eq!(part_2_result, 70);
    }
}
