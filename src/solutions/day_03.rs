use crate::solutions::utils::{FromFile, Solution};
use color_eyre::eyre::{eyre, Result};
use std::collections::HashSet;

pub type Puzzle = Vec<String>;

trait IntoPriority {
    fn into_priority(&self) -> Result<usize>;
}

impl IntoPriority for char {
    fn into_priority(&self) -> Result<usize> {
        match self {
            'a'..='z' => Ok(*self as usize - 'a' as usize + 1),
            'A'..='Z' => Ok(*self as usize - 'A' as usize + 27),
            _ => Err(eyre!("Priority is not defined for `{self}`")),
        }
    }
}

impl FromFile for Puzzle {
    /// Parse lines read from input file into Puzzle.
    fn parse(lines: Vec<String>) -> Result<Self> {
        Ok(lines)
    }
}

impl Solution for Puzzle {
    type Output = u32;

    /// Solution for part 1.
    fn solve1(self) -> Result<Self::Output> {
        let sum: usize = self
            .iter()
            .map(|r| {
                let size = r.len() / 2;
                let mut compartments = [[false, false]; 53];
                for (i, c) in r.chars().enumerate() {
                    compartments[c.into_priority()?][i / size as usize] = true;
                }
                Ok(compartments
                    .iter()
                    .position(|x| *x == [true, true])
                    .ok_or(eyre!(
                        "Failed to find an item that appears in both compartments."
                    ))?)
            })
            .collect::<Result<Vec<_>>>()?
            .iter()
            .sum();
        Ok(sum as u32)
    }

    /// Solution for part 1.
    fn solve2(self) -> Result<Self::Output> {
        let sum = self
            .iter()
            .array_chunks::<3>()
            .map(|x| {
                Ok(x.iter()
                    .map(|x| HashSet::from_iter(x.chars()))
                    .reduce(|acc: HashSet<char>, x| acc.intersection(&x).copied().collect())
                    .unwrap()
                    .iter()
                    .next()
                    .unwrap()
                    .into_priority()? as u32)
            })
            .collect::<Result<Vec<_>>>()?
            .iter()
            .sum();
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
            Puzzle::from_file(format!("testdata/day_03/{stage}/input.txt").as_str()).unwrap();
        let answer = match part {
            1 => puzzle.solve1(),
            2 => puzzle.solve2(),
            _ => unreachable!(),
        }
        .unwrap();

        let expected_path = format!("testdata/day_03/{stage}/output-part{part}.txt");
        let expected = fs::read_to_string(&expected_path)
            .wrap_err(format!(
                "Failed to read expected output from {expected_path}"
            ))
            .unwrap();
        assert_eq!(answer.to_string(), expected.trim());
    }
}
