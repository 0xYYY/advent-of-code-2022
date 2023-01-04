use color_eyre::eyre::{eyre, Error, Result};
use once_cell::sync::Lazy;
use regex::Regex;
use std::str::FromStr;

use crate::solutions::utils::{FromFile, Solution};

static PROCEDURE_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap());

#[derive(Clone, Debug)]
struct Procedure {
    number: usize,
    from: usize,
    to: usize,
}

impl FromStr for Procedure {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = PROCEDURE_RE
            .captures(s)
            .ok_or(eyre!("Failed to match Procedure RegEx: {s}"))?;
        Ok(Self {
            number: captures
                .get(1)
                .ok_or(eyre!("Failed to find number for Procedure: {s}"))?
                .as_str()
                .parse()?,
            from: captures
                .get(2)
                .ok_or(eyre!("Failed to find from for Procedure: {s}"))?
                .as_str()
                .parse::<usize>()?
                - 1,
            to: captures
                .get(3)
                .ok_or(eyre!("Failed to find to for Procedure: {s}"))?
                .as_str()
                .parse::<usize>()?
                - 1,
        })
    }
}

#[derive(Clone)]
pub struct Puzzle {
    stacks: Vec<Vec<char>>,
    procedures: Vec<Procedure>,
}

impl FromFile for Puzzle {
    /// Parse lines read from input file into Puzzle.
    fn parse(lines: Vec<String>) -> Result<Self> {
        let mut lines = lines.iter().enumerate();

        // parse Stacks
        let mut stacks = vec![];
        while let Some((i, line)) = lines.next() {
            let s = line;

            // break and consume empty line when reaching number line
            if s.trim().starts_with('1') {
                lines.next();
                break;
            }

            // init stacks
            if i == 0 {
                stacks = vec![vec![]; (s.len() + 3) / 4];
            }

            // parse crates
            s.chars()
                .enumerate()
                .filter(|(i, _)| i % 4 == 1)
                .for_each(|(i, c)| {
                    if c != ' ' {
                        stacks[i / 4].push(c)
                    }
                });
        }
        for stack in &mut stacks {
            (*stack).reverse();
        }

        // parse Procedures
        let procedures = lines
            .map(|(_, l)| Ok(Procedure::from_str(&l)?))
            .collect::<Result<_>>()?;

        // println!("{:?}", procedures);

        Ok(Self {
            stacks: stacks,
            procedures,
        })
    }
}

impl Puzzle {
    fn rearrange(&mut self, all_at_once: bool) {
        for p in &self.procedures {
            let from = &mut self.stacks[p.from];
            let mut crates = from.split_off(from.len() - p.number);
            if !all_at_once {
                crates.reverse();
            }
            self.stacks[p.to].append(&mut crates);
        }
    }

    fn top(&self) -> Result<String> {
        self.stacks
            .iter()
            .map(|s| s.last().ok_or(eyre!("Empty stack")))
            .collect::<Result<_>>()
    }
}

impl Solution for Puzzle {
    type Output = String;

    /// Solution for part 1.
    fn solve1(mut self) -> Result<Self::Output> {
        self.rearrange(false);
        self.top()
    }

    /// Solution for part 2.
    fn solve2(mut self) -> Result<Self::Output> {
        self.rearrange(true);
        self.top()
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
            Puzzle::from_file(format!("testdata/day_05/{stage}/input.txt").as_str()).unwrap();
        let answer = match part {
            1 => puzzle.solve1(),
            2 => puzzle.solve2(),
            _ => unreachable!(),
        }
        .unwrap();

        let expected_path = format!("testdata/day_05/{stage}/output-part{part}.txt");
        let expected = fs::read_to_string(&expected_path)
            .wrap_err(format!(
                "Failed to read expected output from {expected_path}"
            ))
            .unwrap();
        assert_eq!(answer.to_string(), expected.trim());
    }
}
