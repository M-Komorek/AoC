use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::isize;

fn parse_data(file_path: &str) -> Result<Vec<Vec<usize>>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let data_lines_count = reader.lines().count();
    let mut reports = Vec::with_capacity(data_lines_count);

    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        let levels = line
            .split_whitespace()
            .map(|level| level.parse::<usize>().unwrap())
            .collect();

        reports.push(levels);
    }

    Ok(reports)
}

fn is_report_safe(report: &[usize]) -> bool {
    let mut is_increasing = true;
    let mut is_decreasing = true;

    for window in report.windows(2) {
        let diff = window[1] as isize - window[0] as isize;
        is_increasing &= (1..=3).contains(&diff);
        is_decreasing &= (-3..=-1).contains(&diff);
    }

    is_increasing || is_decreasing
}

fn part_1(file_path: &str) -> Result<u64> {
    let mut valid_reports_count = 0;
    let reports = parse_data(file_path)?;

    for report in reports {
        if is_report_safe(&report) {
            valid_reports_count += 1;
        }
    }

    Ok(valid_reports_count)
}

fn part_2(file_path: &str) -> Result<u64> {
    let mut valid_reports_count = 0;
    let reports = parse_data(file_path)?;

    for report in reports {
        if is_report_safe(&report) {
            valid_reports_count += 1;
        } else {
            let mut found_safe = false;
            for i in 0..report.len() {
                let mut new_report = report.clone();
                new_report.remove(i);

                if is_report_safe(&new_report) {
                    found_safe = true;
                    break;
                }
            }

            if found_safe {
                valid_reports_count += 1;
            }
        }
    }

    Ok(valid_reports_count)
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
        assert_eq!(part_1_result, 2);
    }

    #[test]
    fn expect_part_2_return_correct_result() {
        let part_2_result = part_2("input/test_data").expect("Should return correct value!");
        assert_eq!(part_2_result, 4);
    }
}
