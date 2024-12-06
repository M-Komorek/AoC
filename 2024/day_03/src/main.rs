use std::fs;
use std::io::Result;

fn parse_data(file_path: &str) -> Result<Vec<char>> {
    let file_content = fs::read_to_string(file_path)?;
    let file_content = file_content.replace('\n', "");

    let chars: Vec<char> = file_content.chars().collect();
    Ok(chars)
}

fn part_1(file_path: &str) -> Result<u64> {
    let chars = parse_data(file_path)?;

    let sum = chars
        .windows(4)
        .enumerate()
        .filter_map(|(index, window)| {
            if window == ['m', 'u', 'l', '('] {
                if let Some(close_pos) = chars[index..].iter().position(|&c| c == ')') {
                    let args: String = chars[index + 4..index + close_pos].iter().collect();
                    let values: Vec<&str> = args.split(',').collect();

                    if values.len() == 2 {
                        let x = values[0].trim().parse::<u64>().unwrap_or(0);
                        let y = values[1].trim().parse::<u64>().unwrap_or(0);
                        return Some(x * y);
                    }
                }
            }
            None
        })
        .sum();

    Ok(sum)
}

fn part_2(file_path: &str) -> Result<u64> {
    let chars = parse_data(file_path)?;

    let mut do_flag = true;
    let sum = chars
        .windows(4)
        .enumerate()
        .filter_map(|(index, window)| {
            if index + 8 < chars.len() {
                if chars[index..index + 4] == ['d', 'o', '(', ')'] {
                    do_flag = true;
                }

                if chars[index..index + 7] == ['d', 'o', 'n', '\'', 't', '(', ')'] {
                    do_flag = false;
                }
            }

            if window == ['m', 'u', 'l', '('] {
                if let Some(close_pos) = chars[index..].iter().position(|&c| c == ')') {
                    let args: String = chars[index + 4..index + close_pos].iter().collect();
                    let values: Vec<&str> = args.split(',').collect();

                    if values.len() == 2 {
                        let x = values[0].trim().parse::<u64>().unwrap_or(0);
                        let y = values[1].trim().parse::<u64>().unwrap_or(0);
                        if do_flag {
                            return Some(x * y);
                        }
                    }
                }
            }
            None
        })
        .sum();

    Ok(sum)
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
        assert_eq!(part_1_result, 161);
    }

    #[test]
    fn expect_part_2_return_correct_result() {
        let part_2_result = part_2("input/test_data").expect("Should return correct value!");
        assert_eq!(part_2_result, 48);
    }
}
