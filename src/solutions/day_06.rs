use color_eyre::Result;

fn solve(packets: String, n: usize) -> Result<usize> {
    let mut last_seen = [None; u8::MAX as usize];
    let mut start_pos = 0;
    for (i, c) in packets.bytes().enumerate() {
        if let Some(p) = last_seen[c as usize] && p >= start_pos {
            start_pos = p + 1;
        } else if i - start_pos + 1 == n {
            break;
        }
        last_seen[c as usize] = Some(i);
    }
    Ok(start_pos + n)
}

pub fn part1(input: String) -> Result<usize> {
    solve(input, 4)
}

pub fn part2(input: String) -> Result<usize> {
    solve(input, 14)
}

// #[cfg(test)]
// mod test {
//     use color_eyre::eyre::WrapErr;
//     use std::fs;
//     use test_case::test_case;

//     use super::{part1, part2};

//     #[test_case(1, "sample" ; "sample part1")]
//     // #[test_case(1, "puzzle" ; "puzzle part1")]
//     #[test_case(2, "sample" ; "sample part2")]
//     // #[test_case(2, "puzzle" ; "puzzle part2")]
//     fn test(part: u8, stage: &str) {
//         let result = match part {
//             1 => part1,
//             2 => part2,
//             _ => unreachable!(),
//         }(format!("testdata/{stage}/input.txt").as_str())
//         .unwrap();
//         let expected_path = format!("testdata/{stage}/part{part}-output.txt");
//         let expected = fs::read_to_string(expected_path.as_str())
//             .wrap_err(format!(
//                 "Failed to read expected output from {expected_path}"
//             ))
//             .unwrap();
//         assert_eq!(result, expected.trim().parse().unwrap());
//     }
// }

// fn main() {
//     println!("{}", part1("testdata/puzzle/input.txt").unwrap());
//     println!("{}", part2("testdata/puzzle/input.txt").unwrap());
// }
