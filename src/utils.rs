use clap::Parser;
use color_eyre::eyre::{Result, WrapErr};
use std::{fmt::Display, fs, path::Path};

pub trait Cmd: Parser + Sized {
    async fn run(self) -> Result<()>;
}

pub fn read_file<P>(path: P) -> Result<String>
where
    P: AsRef<Path> + Display + Copy,
{
    fs::read_to_string(path).wrap_err(format!("Failed to read `{path}`"))
}

pub fn write_file<P, C>(path: P, content: C) -> Result<()>
where
    P: AsRef<Path> + Display + Copy,
    C: AsRef<[u8]>,
{
    fs::write(path, content).wrap_err(format!("Failed to update `{path}`"))
}

pub fn fmt_day(day: u8) -> String {
    format!("{:0>2}", day)
}

pub fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
