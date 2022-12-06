use anyhow::{anyhow, Context, Error, Result};
use once_cell::sync::Lazy;
use regex::Regex;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

type Stack = Vec<char>;

struct Procedure {
    number: usize,
    from: usize,
    to: usize,
}

static PROCEDURE_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap());

impl FromStr for Procedure {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = PROCEDURE_RE.captures(s).context("")?;
        Ok(Self {
            number: captures.get(1).context("")?.as_str().parse()?,
            from: captures.get(2).context("")?.as_str().parse::<usize>()? - 1,
            to: captures.get(3).context("")?.as_str().parse::<usize>()? - 1,
        })
    }
}

struct Blueprint {
    stacks: Vec<Stack>,
    procedures: Vec<Procedure>,
}

impl fmt::Display for Procedure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "move {} from {} to {}\n",
            self.number,
            self.from + 1,
            self.to + 1
        )
    }
}

impl Blueprint {
    fn new(path: &str) -> Result<Self> {
        let input_file =
            BufReader::new(File::open(path).context(format!("Failed to open input file {path}"))?);
        let mut lines = input_file.lines().enumerate();

        // parse Stacks
        let mut stacks = vec![];
        while let Some((i, line)) = lines.next() {
            let s = line?;

            // break and consume empty line when reaching number line
            if s.trim().starts_with('1') {
                lines.next();
                break;
            }

            // init stacks
            if i == 0 {
                stacks = vec![vec![]; (s.len() + 3) / 4];
            }

            // parse crates
            s.chars()
                .enumerate()
                .filter(|(i, _)| i % 4 == 1)
                .for_each(|(i, c)| {
                    if c != ' ' {
                        stacks[i / 4].push(c)
                    }
                });
        }
        for stack in &mut stacks {
            (*stack).reverse();
        }

        // parse Procedures
        let procedures = lines
            .map(|(_, l)| Ok(Procedure::from_str(&l.context("failed to parse {l}")?)?))
            .collect::<Result<_>>()?;

        Ok(Self { stacks, procedures })
    }
}

impl fmt::Display for Blueprint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[Stacks]\n")?;
        for (i, stack) in self.stacks.iter().enumerate() {
            write!(f, "{}: {}\n", i + 1, stack.iter().collect::<String>())?;
        }
        write!(f, "\n[Procedures]\n")?;
        for p in &self.procedures {
            write!(f, "{p}")?;
        }
        Ok(())
    }
}

fn rearrange(stacks: &mut Vec<Stack>, p: Procedure, all_at_once: bool) {
    let from = &mut stacks[p.from];
    let mut crates = from.split_off(from.len() - p.number);
    if !all_at_once {
        crates.reverse();
    }
    stacks[p.to].append(&mut crates);
    // println!("[Stacks]");
    // for (i, stack) in stacks.iter().enumerate() {
    //     println!("{}: {}", i + 1, stack.iter().collect::<String>());
    // }
}

pub fn part1(path: &str) -> Result<String> {
    let mut blueprint = Blueprint::new(path).unwrap();
    for p in blueprint.procedures {
        rearrange(&mut blueprint.stacks, p, false);
    }
    blueprint
        .stacks
        .iter()
        .map(|s| s.last().context(""))
        .collect()
}

pub fn part2(path: &str) -> Result<String> {
    let mut blueprint = Blueprint::new(path)?;
    for p in blueprint.procedures {
        rearrange(&mut blueprint.stacks, p, true);
    }
    blueprint
        .stacks
        .iter()
        .map(|s| s.last().context(""))
        .collect()
}

#[cfg(test)]
mod test {
    use anyhow::Context;
    use std::fs;
    use test_case::test_case;

    use super::{part1, part2};

    #[test_case(1, "sample" ; "sample part1")]
    #[test_case(1, "puzzle" ; "puzzle part1")]
    #[test_case(2, "sample" ; "sample part2")]
    #[test_case(2, "puzzle" ; "puzzle part2")]
    fn test(part: u8, stage: &str) {
        let result = match part {
            1 => part1,
            2 => part2,
            _ => unreachable!(),
        }(format!("testdata/{stage}/input.txt").as_str())
        .unwrap();
        let expected_path = format!("testdata/{stage}/part{part}-output.txt");
        let expected = fs::read_to_string(expected_path.as_str())
            .context(format!(
                "Failed to read expected output from {expected_path}"
            ))
            .unwrap();
        assert_eq!(result, expected.trim());
    }
}

fn main() {
    let mut blueprint = Blueprint::new("testdata/puzzle/input.txt").unwrap();
    // println!("{blueprint}");
    for p in blueprint.procedures {
        rearrange(&mut blueprint.stacks, p, true);
    }
    let result: String = blueprint
        .stacks
        .iter()
        .map(|s| s.last().context(""))
        .collect::<Result<String>>()
        .unwrap();
    println!("{result}");
}
