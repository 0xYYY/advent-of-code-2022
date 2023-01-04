use color_eyre::eyre::{eyre, Result};

use crate::solutions::utils::{FromFile, Solution};

#[derive(Clone)]
pub struct Puzzle(String);

impl FromFile for Puzzle {
    /// Parse lines read from input file into Puzzle.
    fn parse(lines: Vec<String>) -> Result<Self> {
        Ok(Self(
            lines.first().ok_or(eyre!("Failed to parse file"))?.into(),
        ))
    }
}

impl Puzzle {
    fn decode(self, n: usize) -> Result<usize> {
        let mut last_seen = [None; u8::MAX as usize];
        let mut start_pos = 0;
        for (i, c) in self.0.bytes().enumerate() {
            if let Some(p) = last_seen[c as usize] && p >= start_pos {
            start_pos = p + 1;
        } else if i - start_pos + 1 == n {
            break;
        }
            last_seen[c as usize] = Some(i);
        }
        Ok(start_pos + n)
    }
}

impl Solution for Puzzle {
    type Output = usize;

    /// Solution for part 1.
    fn solve1(self) -> Result<Self::Output> {
        self.decode(4)
    }

    /// Solution for part 2.
    fn solve2(self) -> Result<Self::Output> {
        self.decode(14)
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
            Puzzle::from_file(format!("testdata/day_06/{stage}/input.txt").as_str()).unwrap();
        let answer = match part {
            1 => puzzle.solve1(),
            2 => puzzle.solve2(),
            _ => unreachable!(),
        }
        .unwrap();

        let expected_path = format!("testdata/day_06/{stage}/output-part{part}.txt");
        let expected = fs::read_to_string(&expected_path)
            .wrap_err(format!(
                "Failed to read expected output from {expected_path}"
            ))
            .unwrap();
        assert_eq!(answer.to_string(), expected.trim());
    }
}
