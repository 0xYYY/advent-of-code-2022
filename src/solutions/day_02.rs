use color_eyre::eyre::{eyre, Error, Result};
use std::str::FromStr;

use crate::solutions::utils::{FromFile, Solution};

#[derive(Clone, Copy)]
#[repr(usize)]
enum Col1 {
    A,
    B,
    C,
}

impl FromStr for Col1 {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Col1::A),
            "B" => Ok(Col1::B),
            "C" => Ok(Col1::C),
            _ => Err(eyre!("Failed to parse {s} as `Col1`")),
        }
    }
}

#[derive(Clone, Copy)]
#[repr(usize)]
enum Col2 {
    X,
    Y,
    Z,
}

impl FromStr for Col2 {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::X),
            "Y" => Ok(Self::Y),
            "Z" => Ok(Self::Z),
            _ => Err(eyre!("Failed to parse {s} as `Col2`")),
        }
    }
}

#[derive(Clone)]
pub struct Round(Col1, Col2);

impl FromStr for Round {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s
            .trim()
            .split_once(' ')
            .ok_or(eyre!("Failed to parse {s} as `Round`"))?;
        Ok(Self(Col1::from_str(split.0)?, Col2::from_str(split.1)?))
    }
}

pub type Puzzle = Vec<Round>;

impl FromFile for Puzzle {
    /// Parse lines read from input file into Puzzle.
    fn parse(lines: Vec<String>) -> Result<Self> {
        lines
            .iter()
            .map(|l| Round::from_str(l))
            .collect::<Result<_>>()
    }
}

impl Solution for Puzzle {
    type Output = u32;

    /// Solution for part 1.
    fn solve1(self) -> Result<Self::Output> {
        let scores: [[u32; 3]; 3] = [[4, 8, 3], [1, 5, 9], [7, 2, 6]];
        let sum = self
            .iter()
            .fold(0, |acc, x| acc + scores[x.0 as usize][x.1 as usize]);

        Ok(sum)
    }

    /// Solution for part 1.
    fn solve2(self) -> Result<Self::Output> {
        let scores: [[u32; 3]; 3] = [[3, 4, 8], [1, 5, 9], [2, 6, 7]];
        let sum = self
            .iter()
            .fold(0, |acc, x| acc + scores[x.0 as usize][x.1 as usize]);

        Ok(sum)
    }
}

#[cfg(test)]
mod test {
    use color_eyre::eyre::WrapErr;
    use std::fs;
    use test_case::test_case;

    use super::Puzzle;
    use crate::solutions::utils::{FromFile, Solution};

    #[test_case("sample", 1 ; "sample part1")]
    #[test_case("puzzle", 1 ; "puzzle part1")]
    #[test_case("sample", 2 ; "sample part2")]
    #[test_case("puzzle", 2 ; "puzzle part2")]
    fn test(stage: &str, part: u8) {
        let puzzle =
            Puzzle::from_file(format!("testdata/day_02/{stage}/input.txt").as_str()).unwrap();
        let answer = match part {
            1 => puzzle.solve1(),
            2 => puzzle.solve2(),
            _ => unreachable!(),
        }
        .unwrap();

        let expected_path = format!("testdata/day_02/{stage}/output-part{part}.txt");
        let expected = fs::read_to_string(&expected_path)
            .wrap_err(format!(
                "Failed to read expected output from {expected_path}"
            ))
            .unwrap();
        assert_eq!(answer.to_string(), expected.trim());
    }
}
