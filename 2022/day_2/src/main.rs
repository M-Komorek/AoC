use std::convert::TryFrom;
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

enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl TryFrom<&u8> for Move {
    type Error = String;

    fn try_from(chunk: &u8) -> Result<Self, Self::Error> {
        match chunk {
            b'A' | b'X' => Ok(Move::Rock),
            b'B' | b'Y' => Ok(Move::Paper),
            b'C' | b'Z' => Ok(Move::Scissors),
            _ => Err(String::from("Invalid bytes!")),
        }
    }
}

enum RoundResult {
    Lose = 0,
    Draw = 3,
    Win = 6,
}

impl TryFrom<&u8> for RoundResult {
    type Error = String;

    fn try_from(chunk: &u8) -> Result<Self, Self::Error> {
        match chunk {
            b'X' => Ok(RoundResult::Lose),
            b'Y' => Ok(RoundResult::Draw),
            b'Z' => Ok(RoundResult::Win),
            _ => Err(String::from("Invalid bytes!")),
        }
    }
}

fn part_1(file_path: &str) -> io::Result<u32> {
    let buffer = read_data_as_bytes(file_path)?;

    use Move::{Paper, Rock, Scissors};
    use RoundResult::{Draw, Lose, Win};

    let score: u32 = buffer.chunks(4).fold(0, |score, moves| {
        let opponent = Move::try_from(&moves[0]).expect("Should be a valid move!");
        let me = Move::try_from(&moves[2]).expect("Should be a valid move!");

        match (opponent, me) {
            (Rock, Rock) => score + Rock as u32 + Draw as u32,
            (Rock, Paper) => score + Paper as u32 + Win as u32,
            (Rock, Scissors) => score + Scissors as u32 + Lose as u32,
            (Paper, Rock) => score + Rock as u32 + Lose as u32,
            (Paper, Paper) => score + Paper as u32 + Draw as u32,
            (Paper, Scissors) => score + Scissors as u32 + Win as u32,
            (Scissors, Rock) => score + Rock as u32 + Win as u32,
            (Scissors, Paper) => score + Paper as u32 + Lose as u32,
            (Scissors, Scissors) => score + Scissors as u32 + Draw as u32,
        }
    });

    Ok(score)
}

fn part_2(file_path: &str) -> io::Result<u32> {
    let buffer = read_data_as_bytes(file_path)?;

    use Move::{Paper, Rock, Scissors};
    use RoundResult::{Draw, Lose, Win};

    let score: u32 = buffer.chunks(4).fold(0, |score, moves| {
        let opponent = Move::try_from(&moves[0]).expect("Should be a valid move!");
        let result = RoundResult::try_from(&moves[2]).expect("Should be a valid move!");

        match (opponent, result) {
            (Rock, Lose) => score + Scissors as u32 + Lose as u32,
            (Rock, Draw) => score + Rock as u32 + Draw as u32,
            (Rock, Win) => score + Paper as u32 + Win as u32,
            (Paper, Lose) => score + Rock as u32 + Lose as u32,
            (Paper, Draw) => score + Paper as u32 + Draw as u32,
            (Paper, Win) => score + Scissors as u32 + Win as u32,
            (Scissors, Lose) => score + Paper as u32 + Lose as u32,
            (Scissors, Draw) => score + Scissors as u32 + Draw as u32,
            (Scissors, Win) => score + Rock as u32 + Win as u32,
        }
    });

    Ok(score)
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
        assert_eq!(part_1_result, 15);
    }

    #[test]
    fn expect_part_2_return_correct_result() {
        let part_2_result = part_2("data/test_data.txt").expect("Should return correct value!");
        assert_eq!(part_2_result, 12);
    }
}
