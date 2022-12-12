use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::Read;

use std::fs;
use std::io;
use std::path::PathBuf;
use std::str::FromStr;

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

fn parse_input(buf_reader: io::BufReader<fs::File>) -> HashMap<PathBuf, u32> {
    let mut dir_to_size = HashMap::new();
    let mut current_dir: Vec<String> = Vec::new();

    for line in buf_reader.lines() {
        let line = line.unwrap();
        if line.starts_with("$ ls") || line.starts_with("dir") {
            continue;
        }

        let command: Vec<&str> = line.split_whitespace().collect();

        match command[..] {
            ["$", "cd", ".."] => {
                current_dir.pop();
            }
            ["$", "cd", name] => {
                current_dir.push(String::from_str(name).unwrap());
            }
            [file_size, _] => {
                let file_size: u32 = file_size.parse().unwrap();
                for dir_id in 0..current_dir.len() {
                    let file_path = PathBuf::from_iter(&current_dir[..=dir_id]);
                    *dir_to_size.entry(file_path).or_insert(0) += file_size;
                }
            }
            _ => unimplemented!(),
        };
    }

    dir_to_size
}

fn part_1(file_path: &str) -> io::Result<u32> {
    let buf_reader = create_buf_reader(file_path)?;
    let dir_to_size = parse_input(buf_reader);

    let accumulate_size_of_dirs_at_most_100000 = dir_to_size
        .into_values()
        .filter(|size| *size <= 100_000)
        .sum::<u32>();

    Ok(accumulate_size_of_dirs_at_most_100000)
}

fn part_2(file_path: &str) -> io::Result<u32> {
    let buf_reader = create_buf_reader(file_path)?;
    let dir_to_size = parse_input(buf_reader);

    let available_size = 70_000_000 - dir_to_size.get(&PathBuf::from_str("/").unwrap()).unwrap();

    let smallest_dir_size = dir_to_size
        .into_values()
        .filter(|size| available_size + size >= 30_000_000)
        .min()
        .unwrap();

    Ok(smallest_dir_size)
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
        assert_eq!(part_1_result, 95437);
    }

    #[test]
    fn expect_part_2_return_correct_result() {
        let part_2_result = part_2("data/test_data.txt").expect("Should return correct value!");
        assert_eq!(part_2_result, 24933642);
    }
}
