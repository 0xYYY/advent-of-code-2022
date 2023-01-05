use clap::Parser;
use color_eyre::eyre::{bail, Result};
use seq_macro::seq;

use crate::solutions::utils::{FromFile, Solution};
use crate::utils;

/// Run the solution of a day.
///
/// Run solutions for both part 1 and 2.
#[derive(Parser, Debug)]
pub struct Args {
    /// Specify the day of the puzzle.
    #[arg(value_parser = clap::value_parser!(u8).range(1..=25))]
    day: u8,
    /// Save the puzzle answers to testdata.
    #[arg(short, long)]
    save: bool,
}

impl utils::Cmd for Args {
    async fn run(self) -> Result<()> {
        seq!(D in 01..=09 {
            match self.day {
                #(D => {
                    use crate::solutions::day_~D::Puzzle;
                    let path = format!("testdata/day_{}/puzzle/input.txt", stringify!(D));
                    let puzzle = Puzzle::from_file(path.as_str())?;
                    puzzle.solve(D, self.save)
                })*
                d => {
                    bail!("Solution for day {d} not yet implemented")
                }
            }
        })
    }
}
