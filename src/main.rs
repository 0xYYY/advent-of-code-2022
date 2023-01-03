#![feature(iter_array_chunks)]
#![feature(let_chains)]
#![feature(async_fn_in_trait)]

use clap::{Parser, Subcommand};
use color_eyre::Result;
use utils::Cmd;

mod init;
mod solutions;
mod solve;
mod utils;

#[derive(Parser)]
pub struct Opts {
    #[clap(subcommand)]
    pub sub: Subcommands,
}

#[derive(Subcommand)]
pub enum Subcommands {
    #[clap(visible_alias = "i")]
    Init(init::Args),
    #[clap(visible_alias = "s")]
    Solve(solve::Args),
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let opts = Opts::parse();
    match opts.sub {
        Subcommands::Init(cmd) => cmd.run().await,
        Subcommands::Solve(cmd) => cmd.run().await,
    }
}
