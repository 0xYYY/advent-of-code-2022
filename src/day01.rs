use anyhow::Result;
use std::fs;
use std::path::Path;

// TODO: remove duplications

pub fn solve1<P: AsRef<Path>>(input_path: P) -> Result<String> {
    let input = fs::read_to_string(input_path)?;
    let nums = input.split('\n').fold(vec![0], |mut acc, x| {
        match x.parse::<u32>() {
            Ok(num) => *acc.last_mut().unwrap() += num,
            Err(_) => acc.push(0),
        };
        acc
    });
    let max = nums.iter().max().unwrap();

    let result = format!("{max}");
    println!("{result}");
    Ok(result)
}

pub fn solve2<P: AsRef<Path>>(input_path: P) -> Result<String> {
    let input = fs::read_to_string(input_path)?;
    let mut nums = input.split('\n').fold(vec![0], |mut acc, x| {
        match x.parse::<u32>() {
            Ok(num) => *acc.last_mut().unwrap() += num,
            Err(_) => acc.push(0),
        };
        acc
    });
    nums.sort();
    nums.reverse();
    let sum: u32 = nums[..3].iter().sum();

    let result = format!("{sum}");
    println!("{result}");
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::{solve1, solve2};
    use std::fs;

    #[test]
    fn test1() {
        let result = solve1("testdata/day01.in.txt").unwrap();
        let output = fs::read_to_string("testdata/day01-1.out.txt").unwrap();
        let expected = output.trim();
        assert_eq!(result, expected);
    }

    #[test]
    fn test2() {
        let result = solve2("testdata/day01.in.txt").unwrap();
        let output = fs::read_to_string("testdata/day01-2.out.txt").unwrap();
        let expected = output.trim();
        assert_eq!(result, expected);
    }
}
