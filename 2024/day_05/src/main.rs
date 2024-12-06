use std::fs;
use std::io::Result;

fn parse_data(file_path: &str) -> Result<(Vec<(u64, u64)>, Vec<Vec<u64>>)> {
    let input = fs::read_to_string(file_path).expect("Failed to read input file");
    let parts: Vec<&str> = input.split("\n\n").collect();

    let rules: Vec<(u64, u64)> = parts[0]
        .lines()
        .map(|line| {
            let mut parts = line.split('|').map(|x| x.parse::<u64>().unwrap());
            (parts.next().unwrap(), parts.next().unwrap())
        })
        .collect();

    let updates: Vec<Vec<u64>> = parts[1]
        .lines()
        .map(|line| line.split(',').map(|x| x.parse::<u64>().unwrap()).collect())
        .collect();

    Ok((rules, updates))
}

fn is_valid_update(update: &[u64], rules: &[(u64, u64)]) -> bool {
    for (x, y) in rules {
        let may_pos_x = update.iter().position(|page| page == x);
        let may_pos_y = update.iter().position(|page| page == y);

        if let (Some(pos_x), Some(pos_y)) = (may_pos_x, may_pos_y) {
            if pos_x >= pos_y {
                return false;
            }
        }
    }

    true
}

fn part_1(file_path: &str) -> Result<u64> {
    let (rules, updates) = parse_data(file_path)?;

    let mut valid_middle_sum = 0;
    for update in updates {
        if is_valid_update(&update, &rules) {
            let middle = update[update.len() / 2];
            valid_middle_sum += middle;
        }
    }

    Ok(valid_middle_sum)
}

fn make_update_correct(update: &[u64], rules: &[(u64, u64)]) -> Vec<u64> {
    let mut correct_update = update.to_vec();

    correct_update.sort_by(|&x, &y| {
        if rules.iter().any(|&(a, b)| a == x && b == y) {
            std::cmp::Ordering::Less
        } else if rules.iter().any(|&(a, b)| a == y && b == x) {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    });

    correct_update
}

fn part_2(file_path: &str) -> Result<u64> {
    let (rules, updates) = parse_data(file_path)?;

    let mut valid_middle_sum = 0;
    for update in updates {
        if !is_valid_update(&update, &rules) {
            let correct_update = make_update_correct(&update, &rules);
            let middle = correct_update[correct_update.len() / 2];
            valid_middle_sum += middle;
        }
    }

    Ok(valid_middle_sum)
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
        assert_eq!(part_1_result, 143);
    }

    #[test]
    fn expect_part_2_return_correct_result() {
        let part_2_result = part_2("input/test_data").expect("Should return correct value!");
        assert_eq!(part_2_result, 123);
    }
}
