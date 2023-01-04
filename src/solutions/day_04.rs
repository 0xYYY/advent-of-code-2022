use color_eyre::eyre::{eyre, Error, Result};
use std::str::FromStr;

use crate::solutions::utils::{FromFile, Solution};

#[derive(Clone)]
struct Sections(i32, i32);

impl Sections {
    fn contains(&self, other: &Self) -> bool {
        self.0 <= other.0 && self.1 >= other.1
    }
}

impl FromStr for Sections {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splits = s
            .split_once('-')
            .ok_or(eyre!("Failed to parse {s} to Sections"))?;
        Ok(Self(splits.0.parse()?, splits.1.parse()?))
    }
}

#[derive(Clone)]
pub struct Pair(Sections, Sections);

impl Pair {
    fn fully_overlap(&self) -> bool {
        self.0.contains(&self.1) || self.1.contains(&self.0)
    }

    fn overlap(&self) -> bool {
        !(self.0 .1 < self.1 .0 || self.1 .1 < self.0 .0)
    }
}

impl FromStr for Pair {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splits = s
            .split_once(',')
            .ok_or(eyre!("Failed to parse {s} to Pair of Sections"))?;
        Ok(Self(
            Sections::from_str(splits.0)?,
            Sections::from_str(splits.1)?,
        ))
    }
}

pub type Puzzle = Vec<Pair>;

impl FromFile for Puzzle {
    /// Parse lines read from input file into Puzzle.
    fn parse(lines: Vec<String>) -> Result<Self> {
        lines.iter().map(|l| l.parse()).collect::<Result<_>>()
    }
}

impl Solution for Puzzle {
    type Output = u32;

    /// Solution for part 1.
    fn solve1(self) -> Result<Self::Output> {
        let sum: u32 = self.iter().map(|p| p.fully_overlap() as u32).sum();
        Ok(sum)
    }

    /// Solution for part 2.
    fn solve2(self) -> Result<Self::Output> {
        let sum: u32 = self.iter().map(|p| p.overlap() as u32).sum();
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
            Puzzle::from_file(format!("testdata/day_04/{stage}/input.txt").as_str()).unwrap();
        let answer = match part {
            1 => puzzle.solve1(),
            2 => puzzle.solve2(),
            _ => unreachable!(),
        }
        .unwrap();

        let expected_path = format!("testdata/day_04/{stage}/output-part{part}.txt");
        let expected = fs::read_to_string(&expected_path)
            .wrap_err(format!(
                "Failed to read expected output from {expected_path}"
            ))
            .unwrap();
        assert_eq!(answer.to_string(), expected.trim());
    }
}
