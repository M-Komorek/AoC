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

fn parse_input(reader: io::BufReader<fs::File>) -> Vec<(u32, u32, u32, u32)> {
    reader
        .lines()
        .map(|line| {
            let line = line.expect("Shoul be valid line of data!");

            let mut number_from_line = line
                .splitn(4, &[',', '-'])
                .map(|substring| substring.parse::<u32>().expect("Should be parsed to u32!"));

            (
                number_from_line.next().unwrap(),
                number_from_line.next().unwrap(),
                number_from_line.next().unwrap(),
                number_from_line.next().unwrap(),
            )
        })
        .collect()
}

fn part_1(file_path: &str) -> io::Result<u32> {
    let reader = create_buf_reader(file_path)?;
    let input_as_numbers = parse_input(reader);

    let overlapping_pairs_sum = input_as_numbers
        .into_iter()
        .filter(|(min_1st, max_1st, min_2nd, max_2nd)| {
            let firts_in_2nd = min_1st >= min_2nd && max_1st <= max_2nd;
            let seconnd_in_1st = min_1st <= min_2nd && max_1st >= max_2nd;

            firts_in_2nd || seconnd_in_1st
        })
        .count();

    Ok(overlapping_pairs_sum as u32)
}

#[allow(dead_code, unused_variables)]
fn part_2(file_path: &str) -> io::Result<u32> {
    let reader = create_buf_reader(file_path)?;
    let input_as_numbers = parse_input(reader);

    let overlapping_pairs_sum = input_as_numbers
        .into_iter()
        .filter(|(min_1st, max_1st, min_2nd, max_2nd)| max_1st >= min_2nd && min_1st <= max_2nd)
        .count();

    Ok(overlapping_pairs_sum as u32)
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
        assert_eq!(part_1_result, 2);
    }

    #[test]
    fn expect_part_2_return_correct_result() {
        let part_2_result = part_2("data/test_data.txt").expect("Should return correct value!");
        assert_eq!(part_2_result, 4);
    }
}
