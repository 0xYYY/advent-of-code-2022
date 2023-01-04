use color_eyre::eyre::{eyre, Result};

use crate::solutions::utils::{FromFile, Solution};

pub type Puzzle = Vec<u32>;

impl FromFile for Puzzle {
    fn parse(lines: Vec<String>) -> Result<Self> {
        let nums = lines.iter().fold(vec![0], |mut acc, x| {
            match x.parse::<u32>() {
                Ok(num) => *acc.last_mut().unwrap() += num,
                Err(_) => acc.push(0),
            };
            acc
        });
        Ok(nums)
    }
}

impl Solution for Puzzle {
    type Output = u32;

    /// Solution for part 1.
    fn solve1(self) -> Result<Self::Output> {
        self.iter()
            .max()
            .ok_or(eyre!("Failed to apply `max`"))
            .copied()
    }

    /// Solution for part 2.
    fn solve2(mut self) -> Result<Self::Output> {
        self.sort();
        self.reverse();
        let sum = self[..3].iter().sum();

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
            Puzzle::from_file(format!("testdata/day_01/{stage}/input.txt").as_str()).unwrap();
        let answer = match part {
            1 => puzzle.solve1(),
            2 => puzzle.solve2(),
            _ => unreachable!(),
        }
        .unwrap();

        let expected_path = format!("testdata/day_01/{stage}/output-part{part}.txt");
        let expected = fs::read_to_string(&expected_path)
            .wrap_err(format!(
                "Failed to read expected output from {expected_path}"
            ))
            .unwrap();
        assert_eq!(answer.to_string(), expected.trim());
    }
}
