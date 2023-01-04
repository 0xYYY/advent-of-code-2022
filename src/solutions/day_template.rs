use color_eyre::eyre::{eyre, Result};

use crate::solutions::utils::{FromFile, Solution};

/// Define the data structure type to be used for this puzzle.
/// The name of the type must be Puzzle.
///
/// # Examples
///
/// ## Type Alias
///
/// pub type Puzzle = Vec<u32>;
///
///
/// ## Struct
///
/// pub struct Puzzle {
///     maze: Vec<Vec<u32>>,
///     start: (u32, u32),
///     end: (u32, u32)
/// }
///
#[derive(Clone)]
pub struct Puzzle;

impl FromFile for Puzzle {
    /// Parse lines read from input file into Puzzle.
    fn parse(lines: Vec<String>) -> Result<Self> {
        // TODO: Remove the following 2 lines and complete the implementation.
        use color_eyre::eyre::bail;
        bail!("not yet implemented")
    }
}

impl Solution for Puzzle {
    type Output = u32;

    /// Solution for part 1.
    fn solve1(self) -> Result<Self::Output> {
        // TODO: Remove the following 2 lines and complete the implementation.
        use color_eyre::eyre::bail;
        bail!("not yet implemented")
    }

    /// Solution for part 2.
    fn solve2(self) -> Result<Self::Output> {
        // TODO: Remove the following 2 lines and complete the implementation.
        use color_eyre::eyre::bail;
        bail!("not yet implemented")
    }
}

#[cfg(test)]
mod test {
    use color_eyre::eyre::WrapErr;
    use std::fs;
    use test_case::test_case;

    use super::Puzzle;
    use crate::solutions::utils::{FromFile, Solution};

    #[test_case("sample", 1 ; "sample part1")]
    #[test_case("puzzle", 1 ; "puzzle part1")]
    #[test_case("sample", 2 ; "sample part2")]
    #[test_case("puzzle", 2 ; "puzzle part2")]
    fn test(stage: &str, part: u8) {
        let puzzle =
            Puzzle::from_file(format!("testdata/day_XX/{stage}/input.txt").as_str()).unwrap();
        let answer = match part {
            1 => puzzle.solve1(),
            2 => puzzle.solve2(),
            _ => unreachable!(),
        }
        .unwrap();

        let expected_path = format!("testdata/day_XX/{stage}/output-part{part}.txt");
        let expected = fs::read_to_string(&expected_path)
            .wrap_err(format!(
                "Failed to read expected output from {expected_path}"
            ))
            .unwrap();
        assert_eq!(answer.to_string(), expected.trim());
    }
}
