use std::fs::File;
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

fn calculate_marks_processed(buffer: &Vec<u8>, window_size: usize) -> u32 {
    // lol, it looks like Rust does not contain out of
    // the box solution to check duplicates in slice
    // https://stackoverflow.com/questions/46766560/how-to-check-if-there-are-duplicates-in-a-slice
    buffer
        .windows(window_size)
        .position(|window| !(1..window.len()).any(|i| window[i..].contains(&window[i - 1])))
        .unwrap() as u32
        + window_size as u32
}

fn part_1(file_path: &str) -> io::Result<u32> {
    let data = read_data_as_bytes(file_path)?;
    Ok(calculate_marks_processed(&data, 4))
}

fn part_2(file_path: &str) -> io::Result<u32> {
    let data = read_data_as_bytes(file_path)?;
    Ok(calculate_marks_processed(&data, 14))
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
        assert_eq!(part_1_result, 10);
    }

    #[test]
    fn expect_part_2_return_correct_result() {
        let part_2_result = part_2("data/test_data.txt").expect("Should return correct value!");
        assert_eq!(part_2_result, 29);
    }
}
