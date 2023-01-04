use color_eyre::eyre::Result;

use crate::solutions::utils::{FromFile, Solution};

#[derive(Clone)]
pub struct Puzzle {
    height: usize,
    width: usize,
    trees: Vec<Vec<u8>>,
    visible: Vec<Vec<bool>>,
}

impl FromFile for Puzzle {
    /// Parse lines read from input file into Puzzle.
    fn parse(lines: Vec<String>) -> Result<Self> {
        let trees: Vec<Vec<u8>> = lines
            .iter()
            .map(|l| l.chars().map(|c| c as u8 - '0' as u8).collect())
            .collect();
        let height = trees.len();
        let width = trees[0].len();
        let visible = vec![vec![false; width]; height];
        Ok(Puzzle {
            height,
            width,
            trees,
            visible,
        })
    }
}

impl Solution for Puzzle {
    type Output = usize;

    /// Solution for part 1.
    fn solve1(mut self) -> Result<Self::Output> {
        for i in 0..self.height {
            let mut max = 0;
            for j in 0..self.width {
                if j == 0 || self.trees[i][j] > max {
                    max = self.trees[i][j];
                    self.visible[i][j] = true;
                }
            }
            max = 0;
            for j in (0..self.width).rev() {
                if j == self.width - 1 || self.trees[i][j] > max {
                    max = self.trees[i][j];
                    self.visible[i][j] = true;
                }
            }
        }
        for j in 0..self.width {
            let mut max = 0;
            for i in 0..self.height {
                if i == 0 || self.trees[i][j] > max {
                    max = self.trees[i][j];
                    self.visible[i][j] = true;
                }
            }
            max = 0;
            for i in (0..self.height).rev() {
                if i == self.height - 1 || self.trees[i][j] > max {
                    max = self.trees[i][j];
                    self.visible[i][j] = true;
                }
            }
        }
        Ok(self
            .visible
            .iter()
            .map(|x| x.iter().filter(|&x| *x).count())
            .sum())
    }

    /// Solution for part 2.
    fn solve2(self) -> Result<Self::Output> {
        let mut max = 0;
        for i in 0..self.height {
            for j in 0..self.width {
                let mut score = 1;
                let mut n = 0;
                for k in (0..j).rev() {
                    n += 1;
                    if self.trees[i][k] >= self.trees[i][j] {
                        break;
                    }
                }
                score *= n;
                n = 0;
                for k in (j + 1)..self.width {
                    n += 1;
                    if self.trees[i][k] >= self.trees[i][j] {
                        break;
                    }
                }
                score *= n;
                n = 0;
                for k in (0..i).rev() {
                    n += 1;
                    if self.trees[k][j] >= self.trees[i][j] {
                        break;
                    }
                }
                score *= n;
                n = 0;
                for k in (i + 1)..self.height {
                    n += 1;
                    if self.trees[k][j] >= self.trees[i][j] {
                        break;
                    }
                }
                score *= n;
                if score > max {
                    max = score;
                }
            }
        }
        Ok(max)
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
            Puzzle::from_file(format!("testdata/day_08/{stage}/input.txt").as_str()).unwrap();
        let answer = match part {
            1 => puzzle.solve1(),

            2 => puzzle.solve2(),
            _ => unreachable!(),
        }
        .unwrap();

        let expected_path = format!("testdata/day_08/{stage}/output-part{part}.txt");
        let expected = fs::read_to_string(&expected_path)
            .wrap_err(format!(
                "Failed to read expected output from {expected_path}"
            ))
            .unwrap();
        assert_eq!(answer.to_string(), expected.trim());
    }
}
