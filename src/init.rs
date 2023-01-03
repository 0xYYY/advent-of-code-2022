use chrono::{DateTime, NaiveDate, Utc};
use clap::Parser;
use color_eyre::eyre::{bail, ensure, eyre, Result, WrapErr};
use regex::Regex;
use reqwest::header;
use scraper::{Html, Selector};
use std::{
    env, fs,
    path::{Path, PathBuf},
};

use crate::utils;

/// Initialize the solution of a day.
///
/// Download puzzle and sample input/output and generate solution from template.
#[derive(Parser, Debug)]
pub struct Args {
    /// Specify the day of the puzzle.
    #[arg(value_parser = clap::value_parser!(u8).range(1..=25))]
    day: u8,
}

impl Args {
    /// Check whether the puzzle is available yet.
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

    /// Download testdata files.
    async fn download(&self) -> Result<()> {
        let (input, output) = self.get_sample().await?;
        println!("[!] Please verify that the following are correct. Otherwise, please manually copy them from the website.\n");
        println!("[Sample Input]");
        println!("{input}\n");
        println!("[Sample Output (Part 1)]");
        println!("{output}\n");
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

        Ok((input.trim_end().into(), output.trim().into()))
    }

    /// Get the puzzle's input.
    async fn get_puzzle(&self) -> Result<String> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::COOKIE,
            format!("session={}", env::var("SESSION")?)
                .parse()
                .wrap_err("Failed to get SESSION env var")?,
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

        Ok(input.trim_end().into())
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

        let path = dir.clone().join("input.txt");
        fs::write(&path, input)?;
        println!(
            "[-] {} input saved to {}.",
            utils::capitalize(stage),
            path.display()
        );

        let path = dir.clone().join("output-part1.txt");
        fs::write(&path, output.clone().unwrap_or("0".into()))?;
        if output.is_some() {
            println!(
                "[-] {} output (part 1) saved to {}.",
                utils::capitalize(stage),
                path.display()
            );
        }

        fs::write(dir.clone().join("output-part2.txt"), "0")?;

        Ok(())
    }

    /// Generate solution from template.
    fn generate(&self) -> Result<()> {
        let day = format!("{:0>2}", self.day);

        // Generate solution file solutions/day_XX.rs.
        let template = utils::read_file("./src/solutions/day_template.rs")?;
        let path = format!("./src/solutions/day_{day}.rs");
        utils::write_file(path.as_str(), template.replace("XX", &day))?;
        println!("[-] Created solution file {path}.");

        // Modify solutions/mod.rs to include the solution module.
        let path = "./src/solutions/mod.rs";
        let content = utils::read_file(path)?;
        let mut lines: Vec<&str> = content.split("\n").collect();
        let mod_str = format!("pub mod day_{day};");
        lines.push(mod_str.as_str());
        lines.sort();
        utils::write_file(path, lines.join("\n").trim())?;

        // Modify solve.rs to add an match arm for the day.
        let path = "./src/solve.rs";
        let mut content = utils::read_file(path)?;
        let re = Regex::new(r"seq!\(D in 01..=\d{2} \{")?;
        content = re
            .replace(&content, format!("seq!(D in 01..={day} {{"))
            .to_string();
        utils::write_file(path, content)?;

        Ok(())
    }
}

impl utils::Cmd for Args {
    async fn run(self) -> Result<()> {
        // Check the puzzle is available
        ensure!(
            self.validate_day(),
            "Puzzle for day {} is not available yet",
            self.day
        );

        // Check that files to be created don't already exist
        let day = format!("{:0>2}", self.day);
        let paths = [
            format!("./src/solutions/day_{day}.rs"),
            format!("./testdata/day_{day}/"),
        ];
        for p in paths {
            if Path::new(&p).exists() {
                bail!("`{p}` already exists, remove to continue")
            }
        }

        // Download test data and generate solution sketch
        self.download().await?;
        self.generate()?;

        Ok(())
    }
}
