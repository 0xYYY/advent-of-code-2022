use color_eyre::eyre::{Result, WrapErr};
use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use crate::utils;

pub trait FromFile: Sized {
    fn parse(lines: Vec<String>) -> Result<Self>;
    fn from_file<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path> + Display + Copy,
    {
        let lines = BufReader::new(
            File::open(path).wrap_err(format!("Failed to open input file `{path}`"))?,
        )
        .lines()
        .enumerate()
        .map(|(i, l)| l.wrap_err(format!("Failed to read line {i} from `{path}`")))
        .collect::<Result<Vec<String>>>()?;
        Self::parse(lines)
    }
}

pub trait Solution: Clone {
    type Output: Display;

    fn solve1(self) -> Result<Self::Output>;
    fn solve2(self) -> Result<Self::Output>;
    fn solve(self, day: u8, save: bool) -> Result<()> {
        let day = utils::fmt_day(day);
        println!("================ Day {day} ================");

        print!("[Part 1] ");
        let ans1 = self
            .clone()
            .solve1()
            .wrap_err("Failed to run solution for part 1")?;
        println!("Answer: {ans1}");

        if save {
            let path = format!("./testdata/day_{day}/puzzle/output-part1.txt");
            utils::write_file(path.as_str(), ans1.to_string())?;
            println!("(Saved to {path})");
        }

        print!("\n[Part 2] ");
        let ans2 = self
            .solve2()
            .wrap_err("Failed to run solution for part 2")?;
        println!("Answer: {ans2}");

        if save {
            let path = format!("./testdata/day_{day}/puzzle/output-part2.txt");
            utils::write_file(path.as_str(), ans2.to_string())?;
            println!("(Saved to {path})");
        }

        Ok(())
    }
}
