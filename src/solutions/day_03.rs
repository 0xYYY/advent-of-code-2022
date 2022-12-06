use anyhow::{anyhow, Context, Result};
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn read_input<P: AsRef<Path>>(input_path: P) -> Result<Vec<String>> {
    let file = File::open(input_path)?;
    Ok(BufReader::new(file).lines().collect::<Result<_, _>>()?)
}

trait IntoPriority {
    fn into_priority(&self) -> Result<usize>;
}

impl IntoPriority for char {
    fn into_priority(&self) -> Result<usize> {
        match self {
            'a'..='z' => Ok(*self as usize - 'a' as usize + 1),
            'A'..='Z' => Ok(*self as usize - 'A' as usize + 27),
            _ => Err(anyhow!("Priority is not defined for `{self}`")),
        }
    }
}

pub fn solve1<P: AsRef<Path>>(input_path: P) -> Result<String> {
    let rucksacks = read_input(input_path)?;

    let sum: usize = rucksacks
        .iter()
        .map(|r| {
            let size = r.len() / 2;
            let mut compartments = [[false, false]; 53];
            for (i, c) in r.chars().enumerate() {
                compartments[c.into_priority()?][i / size as usize] = true;
            }
            Ok(compartments
                .iter()
                .position(|x| *x == [true, true])
                .ok_or_else(|| anyhow!("a"))?)
        })
        .collect::<Result<Vec<usize>, anyhow::Error>>()?
        .iter()
        .sum();

    let result = format!("{sum}");
    println!("{sum}");
    Ok(result)
}

pub fn solve2<P: AsRef<Path>>(input_path: P) -> Result<String> {
    let rucksacks = read_input(input_path)?;
    let sum: u32 = rucksacks
        .iter()
        .array_chunks::<3>()
        .map(|x| {
            Ok(x.iter()
                .map(|x| HashSet::from_iter(x.chars()))
                .reduce(|acc: HashSet<char>, x| acc.intersection(&x).copied().collect())
                .context("a")?
                .iter()
                .next()
                .context("a")?
                .into_priority()? as u32)
        })
        .collect::<Result<Vec<_>>>()?
        .iter()
        .sum();

    let result = format!("{sum}");
    println!("{sum}");
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::{solve1, solve2};
    use std::fs;

    #[test]
    fn test1() {
        let result = solve1("testdata/day02.in.txt").unwrap();
        let output = fs::read_to_string("testdata/day02-1.out.txt").unwrap();
        let expected = output.trim();
        assert_eq!(result, expected);
    }

    #[test]
    fn test2() {
        let result = solve2("testdata/day02.in.txt").unwrap();
        let output = fs::read_to_string("testdata/day02-2.out.txt").unwrap();
        let expected = output.trim();
        assert_eq!(result, expected);
    }
}
