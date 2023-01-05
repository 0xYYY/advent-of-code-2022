use color_eyre::eyre::{bail, eyre, Context, Result};
use std::collections::HashSet;
use std::ops::{Add, AddAssign, Mul, Sub};

use crate::solutions::utils::{FromFile, Solution};

#[derive(Debug, Clone, Default, Copy, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub const UNIT: Self = Self { x: 1, y: 1 };

    fn signum(&self) -> Self {
        Self {
            x: self.x.signum(),
            y: self.y.signum(),
        }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul for Point {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

#[derive(Clone, Copy)]
enum Dir {
    R,
    L,
    U,
    D,
}

#[derive(Clone)]
struct Motion {
    dir: Dir,
    num: usize,
}

#[derive(Clone, Default)]
pub struct Puzzle {
    rope: Vec<Point>,
    motions: Vec<Motion>,
    visited: HashSet<Point>,
}

impl Puzzle {
    fn new(motions: Vec<Motion>) -> Self {
        Self {
            motions,
            ..Self::default()
        }
    }

    fn set_rope_length(&mut self, n: usize) {
        self.rope = vec![Point::default(); n];
    }

    fn r#move(&mut self) {
        for m in self.motions.iter() {
            for _ in 0..m.num {
                self.rope[0] += match m.dir {
                    Dir::R => Point { x: 1, y: 0 },
                    Dir::L => Point { x: -1, y: 0 },
                    Dir::U => Point { x: 0, y: 1 },
                    Dir::D => Point { x: 0, y: -1 },
                };
                for i in 1..self.rope.len() {
                    let diff = self.rope[i - 1] - self.rope[i];
                    if diff.x.abs() >= 2 || diff.y.abs() >= 2 {
                        self.rope[i] += Point::UNIT * diff.signum();
                    }
                    self.visited.insert(*self.rope.last().unwrap());
                }
            }
        }
    }

    fn count_visited(&self) -> usize {
        self.visited.len()
    }
}

impl FromFile for Puzzle {
    /// Parse lines read from input file into Puzzle.
    fn parse(lines: Vec<String>) -> Result<Self> {
        let moves = lines
            .iter()
            .enumerate()
            .map(|(i, l)| {
                let splits = l
                    .split_once(' ')
                    .ok_or(eyre!("Failed to parse line {i}: {l}"))?;
                let dir = match splits.0 {
                    "R" => Dir::R,
                    "L" => Dir::L,
                    "U" => Dir::U,
                    "D" => Dir::D,
                    _ => bail!("Failed to parse line {i}: {l}"),
                };
                let num = splits.1.parse().wrap_err("Failed to parse line {i}: {l}")?;
                Ok(Motion { dir, num })
            })
            .collect::<Result<_>>()?;
        Ok(Self::new(moves))
    }
}

impl Solution for Puzzle {
    type Output = usize;

    /// Solution for part 1.
    fn solve1(mut self) -> Result<Self::Output> {
        self.set_rope_length(2);
        self.r#move();
        Ok(self.count_visited())
    }

    /// Solution for part 2.
    fn solve2(mut self) -> Result<Self::Output> {
        self.set_rope_length(10);
        self.r#move();
        Ok(self.count_visited())
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
            Puzzle::from_file(format!("testdata/day_09/{stage}/input.txt").as_str()).unwrap();
        let answer = match part {
            1 => puzzle.solve1(),
            2 => puzzle.solve2(),
            _ => unreachable!(),
        }
        .unwrap();

        let expected_path = format!("testdata/day_09/{stage}/output-part{part}.txt");
        let expected = fs::read_to_string(&expected_path)
            .wrap_err(format!(
                "Failed to read expected output from {expected_path}"
            ))
            .unwrap();
        assert_eq!(answer.to_string(), expected.trim());
    }
}
