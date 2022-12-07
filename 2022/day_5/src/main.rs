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

struct Instruction {
    quantity: u8,
    from: u8,
    to: u8,
}

struct ParsedData {
    stacks: Vec<Vec<char>>,
    instructions: Vec<Instruction>,
}

fn parse_stacks(unparsed_stacks: &str, unparsed_stacks_number: &str) -> Vec<Vec<char>> {
    let stucks_number = unparsed_stacks_number
        .chars()
        .filter(|mark| !mark.is_whitespace())
        .count();

    let mut stacks = vec![Vec::new(); stucks_number];
    for line in unparsed_stacks.lines().rev() {
        for (id, chunk) in line.as_bytes().chunks(4).enumerate() {
            let stuck_name = chunk[1];
            if stuck_name != ' ' as u8 {
                stacks[id].push(stuck_name as char);
            }
        }
    }

    stacks
}

fn parse_to_u8_from_chars(data: &String, position: usize) -> u8 {
    data.chars()
        .nth(position)
        .expect("Should contain data!")
        .to_digit(10)
        .expect("Should be parsed to u8!") as u8
}

fn parse_instructions(unparsed_instructions: &str) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    for line in unparsed_instructions.lines() {
        let data: String = line
            .chars()
            .filter(|mark| !mark.is_ascii_alphabetic() && !mark.is_whitespace())
            .collect();

        let (quantity, from, to) = match data.len() {
            3 => (
                parse_to_u8_from_chars(&data, 0),
                parse_to_u8_from_chars(&data, 1) - 1,
                parse_to_u8_from_chars(&data, 2) - 1,
            ),
            4 => {
                let quantity_str = data.chars().nth(0).unwrap().to_string()
                    + &data.chars().nth(1).unwrap().to_string();
                (
                    quantity_str.parse::<u8>().unwrap(),
                    parse_to_u8_from_chars(&data, 2) - 1,
                    parse_to_u8_from_chars(&data, 3) - 1,
                )
            }
            _ => unreachable!(),
        };

        let instruction = Instruction { quantity, from, to };
        instructions.push(instruction);
    }

    instructions
}

fn parse_input(mut reader: io::BufReader<fs::File>) -> Option<ParsedData> {
    let mut input = String::new();
    reader.read_to_string(&mut input).unwrap();

    let (unparsed_stacks_and_crates, unparsed_instructions) = input.split_once("\n\n")?;
    let (unparsed_stacks, unparsed_stacks_number) = unparsed_stacks_and_crates.rsplit_once("\n")?;

    let stacks = parse_stacks(unparsed_stacks, unparsed_stacks_number);
    let instructions = parse_instructions(unparsed_instructions);

    Some(ParsedData {
        stacks,
        instructions,
    })
}

fn part_1(file_path: &str) -> io::Result<String> {
    let reader = create_buf_reader(file_path)?;
    let mut parsed_data = parse_input(reader).unwrap();

    for instruction in parsed_data.instructions {
        for _ in 0..instruction.quantity {
            if let Some(removed) = parsed_data.stacks[instruction.from as usize].pop() {
                parsed_data.stacks[instruction.to as usize].push(removed);
            }
        }
    }

    let result = parsed_data
        .stacks
        .into_iter()
        .filter_map(|stack| stack.into_iter().last())
        .collect();

    Ok(result)
}

fn part_2(file_path: &str) -> io::Result<String> {
    let reader = create_buf_reader(file_path)?;
    let mut parsed_data = parse_input(reader).unwrap();

    for instruction in parsed_data.instructions {
        let stack_len = parsed_data.stacks[instruction.from as usize].len();
        let removed = parsed_data.stacks[instruction.from as usize]
            .split_off(stack_len - instruction.quantity as usize);

        parsed_data.stacks[instruction.to as usize].extend(removed);
    }

    let result = parsed_data
        .stacks
        .into_iter()
        .filter_map(|stack| stack.into_iter().last())
        .collect();

    Ok(result)
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
        assert_eq!(part_1_result, "CMZ");
    }

    #[test]
    fn expect_part_2_return_correct_result() {
        let part_2_result = part_2("data/test_data.txt").expect("Should return correct value!");
        assert_eq!(part_2_result, "MCD");
    }
}
