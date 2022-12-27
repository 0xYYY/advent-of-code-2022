#![feature(iter_array_chunks)]
#![feature(let_chains)]
#![feature(async_fn_in_trait)]

use crate::utils::Cmd;
use clap::{Parser, Subcommand};
use color_eyre::Result;

mod scaffold;
mod solutions;
mod utils;

// TODO: remove duplications with macro

#[derive(Parser)]
pub struct Opts {
    #[clap(subcommand)]
    pub sub: Subcommands,
}

#[derive(Subcommand)]
pub enum Subcommands {
    #[clap(visible_alias = "s")]
    Scaffold(scaffold::Args),
    #[clap(visible_alias = "r")]
    Run,
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let opts = Opts::parse();
    match opts.sub {
        Subcommands::Scaffold(cmd) => cmd.run().await,
        Subcommands::Run => {
            println!("run");
            Ok(())
        }
    }

    // solutions::day_01::solve1("testdata/day_01/puzzle/input.txt").unwrap();
    // solutions::day_01::solve2("testdata/day_01/puzzle/input.txt").unwrap();
    // solutions::day_02::solve1("testdata/day_02/puzzle/input.txt").unwrap();
    // solutions::day_02::solve2("testdata/day_02/puzzle/input.txt").unwrap();
    // solutions::day_03::solve1("testdata/day_03/puzzle/input.txt").unwrap();
    // solutions::day_03::solve2("testdata/day_03/puzzle/input.txt").unwrap();
    // solutions::day_04::solve1(
    //     lib::InputPath(String::from("testdata/day_04/puzzle/input.txt"))
    //         .try_into()
    //         .unwrap(),
    // )
    // .unwrap();
    // solutions::day_04::solve2(
    //     lib::InputPath(String::from("testdata/day_04/puzzle/input.txt"))
    //         .try_into()
    //         .unwrap(),
    // )
    // .unwrap();
    // println!(
    //     "{}",
    //     solutions::day_05::part1("testdata/day_05/puzzle/input.txt")?
    // );
    // println!(
    //     "{}",
    //     solutions::day_05::part2("testdata/day_05/puzzle/input.txt")?
    // );
    // let input = utils::Input::new("testdata/day_06/puzzle/input.txt")?;
    // println!("{}", solutions::day_06::part1(input.try_into()?)?);

    // Ok(())
}
