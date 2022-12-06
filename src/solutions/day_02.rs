use anyhow::{anyhow, Context, Result};
use std::fs;
use std::path::Path;
use std::str::FromStr;

#[derive(Clone, Copy)]
#[repr(usize)]
enum Col1 {
    A,
    B,
    C,
}

impl FromStr for Col1 {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Col1::A),
            "B" => Ok(Col1::B),
            "C" => Ok(Col1::C),
            _ => Err(anyhow!("Failed to parse `{s}` as `Col1`")),
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
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::X),
            "Y" => Ok(Self::Y),
            "Z" => Ok(Self::Z),
            _ => Err(anyhow!("Failed to parse `{s}` as `Col2`")),
        }
    }
}

struct Round2(Col1, Col2);

impl FromStr for Round2 {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s
            .trim()
            .split_once(' ')
            .context("Failed to parse `{s}` as `Round`")?;
        Ok(Self(Col1::from_str(split.0)?, Col2::from_str(split.1)?))
    }
}

fn read_input<P: AsRef<Path>>(input_path: P) -> Result<Vec<Round2>> {
    let input = fs::read_to_string(input_path)?;
    Ok(input
        .trim()
        .split('\n')
        .map(Round2::from_str)
        .collect::<Result<_, _>>()?)
}

pub fn solve1<P: AsRef<Path>>(input_path: P) -> Result<String> {
    let rounds = read_input(input_path)?;

    let scores: [[u32; 3]; 3] = [[4, 8, 3], [1, 5, 9], [7, 2, 6]];
    let sum = rounds
        .iter()
        .fold(0, |acc, x| acc + scores[x.0 as usize][x.1 as usize]);

    let result = format!("{sum}");
    println!("{sum}");
    Ok(result)
}

pub fn solve2<P: AsRef<Path>>(input_path: P) -> Result<String> {
    let rounds = read_input(input_path)?;

    let scores: [[u32; 3]; 3] = [[3, 4, 8], [1, 5, 9], [2, 6, 7]];
    let sum = rounds
        .iter()
        .fold(0, |acc, x| acc + scores[x.0 as usize][x.1 as usize]);

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
        let result = solve1("testdata/day02.in.txt").unwrap();
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
