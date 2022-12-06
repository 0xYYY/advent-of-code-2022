use color_eyre::eyre::{self, eyre, Result, WrapErr};
use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

pub struct Input<R: BufRead> {
    inner: R,
}

impl Input<BufReader<File>> {
    pub fn new(path: &str) -> Result<Self> {
        Ok(Self {
            inner: BufReader::new(File::open(path).wrap_err("Failed to open input file `{path}`")?),
        })
    }
}

impl<R: BufRead, T: FromStr> TryFrom<Input<R>> for Vec<T>
where
    T::Err: Display,
{
    type Error = eyre::Error;

    fn try_from(input: Input<R>) -> Result<Self, Self::Error> {
        input
            .inner
            .lines()
            .enumerate()
            .map(|(line_number, line)| {
                T::from_str(&line?)
                    .map_err(|e| eyre!("Failed to parse line {}: {}", line_number + 1, e))
            })
            .collect()
    }
}

impl<R: BufRead> TryFrom<Input<R>> for String {
    type Error = eyre::Error;

    fn try_from(mut input: Input<R>) -> Result<Self, Self::Error> {
        let mut result = String::new();
        input.inner.read_to_string(&mut result)?;
        Ok(result.trim().into())
    }
}
