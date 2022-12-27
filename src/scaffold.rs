use chrono::{DateTime, NaiveDate, Utc};
use clap::Parser;
use color_eyre::eyre::{ensure, eyre, Result};
use reqwest::header;
use scraper::{Html, Selector};
use std::{env, fs, path::PathBuf};

use crate::utils::Cmd;

/// Scaffold the solution for a day.
///
/// This will generate:
///     1. solutions/day_XX.rs: contains solution functions for both parts and tests,
///     2. testdata/day_XX/: input and output files for both parts of the sample and the actual puzzle.
#[derive(Parser, Debug)]
pub struct Args {
    /// Specify the day of the puzzle.
    #[arg(value_parser = clap::value_parser!(u8).range(1..=25))]
    day: u8,
}

impl Args {
    /// Check whether if the puzzle is available yet.
    /// New puzzle is released at UTC 05:00 each day.
    fn validate_day(&self) -> bool {
        let day = DateTime::<Utc>::from_utc(
            NaiveDate::from_ymd_opt(2022, 12, self.day.into())
                .unwrap()
                .and_hms_opt(5, 0, 0)
                .unwrap(),
            Utc,
        );
        let now = Utc::now();
        now >= day
    }

    /// Create testdata files.
    async fn create_testdata(&self) -> Result<()> {
        let (input, output) = self.get_sample().await?;
        self.create_testdata_files("sample", input, Some(output))?;

        let input = self.get_puzzle().await?;
        self.create_testdata_files("puzzle", input, None)?;

        Ok(())
    }

    /// Get the puzzle's sample input and output (for part 1).
    async fn get_sample(&self) -> Result<(String, String)> {
        let body = reqwest::get(format!("https://adventofcode.com/2022/day/{}", self.day))
            .await?
            .text()
            .await?;
        let doc = Html::parse_document(&body);

        let input_sel = Selector::parse("pre > code").unwrap();
        let input = doc
            .select(&input_sel)
            .next()
            .ok_or(eyre!("Can't find sample input element"))?
            .text()
            .next()
            .ok_or(eyre!("Can't find sample input text"))?;

        let output_sel = Selector::parse("code > em").unwrap();
        let output = doc
            .select(&output_sel)
            .last()
            .ok_or(eyre!("Can't find sample output element"))?
            .text()
            .next()
            .ok_or(eyre!("Can't find sample output text"))?;

        Ok((input.trim().into(), output.trim().into()))
    }

    /// Get the puzzle's input.
    async fn get_puzzle(&self) -> Result<String> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::COOKIE,
            format!("session={}", env::var("SESSION")?).parse()?,
        );

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        let input = client
            .get(format!(
                "https://adventofcode.com/2022/day/{}/input",
                self.day
            ))
            .send()
            .await?
            .text()
            .await?;

        Ok(input.trim().into())
    }

    /// Create testdata input and output files.
    fn create_testdata_files(
        &self,
        stage: &str,
        input: String,
        output: Option<String>,
    ) -> Result<()> {
        let dir = PathBuf::from(format!("testdata/day_{:0>2}/{stage}/", self.day));
        fs::create_dir_all(dir.clone())?;
        fs::write(dir.clone().join("input.txt"), input)?;
        fs::write(
            dir.clone().join("output-part1.txt"),
            output.unwrap_or("0".into()),
        )?;
        fs::write(dir.clone().join("output-part2.txt"), "0")?;

        Ok(())
    }
}

impl Cmd for Args {
    async fn run(self) -> Result<()> {
        ensure!(
            self.validate_day(),
            "Puzzle for day {} is not available yet",
            self.day
        );
        self.create_testdata().await?;
        // TODO: generate template for solutions/day_XX.rs

        Ok(())
    }
}
