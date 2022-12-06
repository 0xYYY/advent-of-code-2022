use chrono::{Datelike, Timelike, Utc};
use clap::Parser;
use color_eyre::eyre::{ensure, eyre, Result};
use scraper::{Html, Selector};

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    day: u32,
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let args = Args::parse();

    // validate day
    let now = Utc::now();
    ensure!(
        args.day < now.day() || (args.day == now.day() && now.hour() >= 5),
        "Challenge for day {} is not available yet",
        args.day
    );

    let body = reqwest::get(format!("https://adventofcode.com/2022/day/{}", args.day))
        .await?
        .text()
        .await?;

    let doc = Html::parse_document(&body);
    let sel = Selector::parse("pre > code").unwrap();

    let input = doc
        .select(&sel)
        .next()
        .ok_or(eyre!("Can't find pre > code element"))?
        .text()
        .next()
        .ok_or(eyre!(
            "Can't find text segment in the first pre > code element"
        ))?;

    // TODO: save to testdata
    println!("{}", input);

    Ok(())
}
