use anyhow::{Context, Result};
use std::path::Path;
use std::str::FromStr;

struct Sections(i32, i32);

impl Sections {
    fn contains(&self, other: &Self) -> bool {
        self.0 <= other.0 && self.1 >= other.1
    }
}

impl FromStr for Sections {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splits = s.split_once('-').context("a")?;
        Ok(Self(splits.0.parse()?, splits.1.parse()?))
    }
}

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
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splits = s.split_once(',').context("a")?;
        Ok(Self(
            Sections::from_str(splits.0)?,
            Sections::from_str(splits.1)?,
        ))
    }
}

pub fn solve1(input: Vec<Pair>) -> Result<String> {
    let sum: u32 = input.iter().map(|p| p.fully_overlap() as u32).sum();

    let result = format!("{sum}");
    println!("{sum}");
    Ok(result)
}

pub fn solve2(input: Vec<Pair>) -> Result<String> {
    let sum: u32 = input.iter().map(|p| p.overlap() as u32).sum();

    let result = format!("{sum}");
    println!("{sum}");
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::{solve1, solve2};
    use std::fs;

    #[test]
    fn test1() {
        let result = solve1("testdata/day03.in.txt").unwrap();
        let output = fs::read_to_string("testdata/day02-1.out.txt").unwrap();
        let expected = output.trim();
        assert_eq!(result, expected);
    }

    #[test]
    fn test2() {
        let result = solve2("testdata/day02.in.txt").unwrap();
        let output = fs::read_to_string("testdata/day02-2.out.txt").unwrap();
        let expected = output.trim();
        assert_eq!(result, expected);
    }
}
