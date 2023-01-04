use camino::Utf8PathBuf;
use color_eyre::eyre::{eyre, Result, WrapErr};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::u64,
    combinator::{all_consuming, map},
    sequence::{preceded, separated_pair},
    Finish, IResult,
};
use std::cell::RefCell;
use std::collections::{hash_map, HashMap};
use std::rc::Rc;

use crate::solutions::utils::{FromFile, Solution};

const PATHCHARS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ./";

fn parse_path(i: &str) -> IResult<&str, Utf8PathBuf> {
    map(take_while1(|c: char| PATHCHARS.contains(c)), Into::into)(i)
}

#[derive(Clone)]
struct Ls;

impl Ls {
    fn parse(i: &str) -> IResult<&str, Self> {
        map(tag("ls"), |_| Self)(i)
    }
}

#[derive(Clone)]
struct Cd(Utf8PathBuf);

impl Cd {
    fn parse(i: &str) -> IResult<&str, Self> {
        map(preceded(tag("cd "), parse_path), Cd)(i)
    }
}

#[derive(Clone)]
enum Cmd {
    Ls(Ls),
    Cd(Cd),
}

impl Cmd {
    fn parse(i: &str) -> IResult<&str, Self> {
        let (i, _) = tag("$ ")(i)?;
        alt((map(Ls::parse, Cmd::Ls), map(Cd::parse, Cmd::Cd)))(i)
    }
}

#[derive(Clone, Debug)]
struct Dir {
    path: Utf8PathBuf,
}

impl Dir {
    fn parse(i: &str) -> IResult<&str, Self> {
        map(preceded(tag("dir "), parse_path), |path| Dir { path })(i)
    }
}

#[derive(Clone, Debug)]
struct File {
    path: Utf8PathBuf,
    size: u64,
}

impl File {
    fn parse(i: &str) -> IResult<&str, Self> {
        map(separated_pair(u64, tag(" "), parse_path), |(size, path)| {
            File { size, path }
        })(i)
    }
}

#[derive(Clone, Debug)]
enum Entry {
    Dir(Dir),
    File(File),
}

impl Entry {
    fn parse(i: &str) -> IResult<&str, Self> {
        alt((map(Dir::parse, Entry::Dir), map(File::parse, Entry::File)))(i)
    }

    fn path(&self) -> Utf8PathBuf {
        match self {
            Self::Dir(d) => d.path.clone(),
            Self::File(f) => f.path.clone(),
        }
    }
}

#[derive(Clone)]
enum Line {
    Cmd(Cmd),
    Entry(Entry),
}

impl Line {
    fn parse(i: &str) -> IResult<&str, Self> {
        alt((map(Cmd::parse, Line::Cmd), map(Entry::parse, Line::Entry)))(i)
    }
}

#[derive(Clone)]
struct Node {
    entry: Entry,
    parent: Option<NodeHandle>,
    children: HashMap<Utf8PathBuf, NodeHandle>,
}

type NodeHandle = Rc<RefCell<Node>>;

impl Node {
    fn is_dir(&self) -> bool {
        matches!(self.entry, Entry::Dir(_))
    }

    fn size(&self) -> u64 {
        match &self.entry {
            Entry::File(f) => f.size,
            Entry::Dir(_) => self.children.values().map(|c| c.borrow().size()).sum(),
        }
    }
}

#[derive(Clone)]
pub struct Puzzle {
    // term: Vec<Line>,
    tree: NodeHandle,
}

impl FromFile for Puzzle {
    /// Parse lines read from input file into Puzzle.
    fn parse(lines: Vec<String>) -> Result<Self> {
        // HACK: Deliberately leak memory to make lines to have 'static lifetime.
        let lines = Box::leak(Box::new(lines));
        let term = lines
            .iter()
            .map(|l| {
                Ok(all_consuming(Line::parse)(l)
                    .finish()
                    .wrap_err("Failed to parse input")?
                    .1)
            })
            .collect::<Result<Vec<_>>>()?;

        let root = Rc::new(RefCell::new(Node {
            entry: Entry::Dir(Dir { path: "/".into() }),
            parent: None,
            children: HashMap::new(),
        }));

        let mut node = root.clone();
        for line in term {
            match line {
                Line::Cmd(cmd) => match cmd {
                    Cmd::Ls(_) => {
                        // Do nothing, as the listed entries will be handled in their respected
                        // lines.
                    }
                    Cmd::Cd(Cd(path)) => {
                        match path.as_str() {
                            "/" => {
                                // Do nothing, since we already initilized the root node as /
                                // directory. (This line only appears once at the very beginning in
                                // this puzzle.)
                            }
                            ".." => {
                                let parent = node.borrow().parent.clone();
                                node = parent.ok_or(eyre!("Failed to `cd ..`"))?;
                            }
                            _ => {
                                let child = match node.borrow_mut().children.entry(path.clone()) {
                                    hash_map::Entry::Vacant(_) => {
                                        Err(eyre!("Failed to `cd {path}`"))
                                    }
                                    hash_map::Entry::Occupied(e) => Ok(e.get().clone()),
                                }?;
                                node = child
                            }
                        }
                    }
                },
                Line::Entry(entry) => {
                    let path = entry.path();
                    let child = Rc::new(RefCell::new(Node {
                        entry,
                        children: HashMap::new(),
                        parent: Some(node.clone()),
                    }));
                    node.borrow_mut().children.insert(path, child);
                }
            }
        }

        Ok(Puzzle { tree: root })
    }
}

impl Puzzle {
    /// Returns tuple of (size, ans)
    fn traverse1(node: NodeHandle) -> (u64, u64) {
        let n = node.borrow();

        // Node is a file entry.
        if !n.is_dir() {
            return (n.size(), 0);
        }

        // size is the combined size of this node's children. ans is the sum of the size of the
        // descendent dirs that matches the condition (size <= 100000).
        let (size, mut ans) = n
            .children
            .values()
            .map(|c| Self::traverse1(c.clone()))
            .fold((0, 0), |acc, x| (acc.0 + x.0, acc.1 + x.1));
        if n.is_dir() && size <= 100_000 {
            ans += size;
        }

        return (size, ans);
    }

    /// Returns tuple of (size, ans)
    fn traverse2(node: NodeHandle, target: u64) -> (u64, u64) {
        let n = node.borrow();

        // Node is a file entry.
        if !n.is_dir() {
            return (n.size(), u64::MAX);
        }

        // size is the combined size of this node's children. ans is the size of a descendent
        // dir that matches the condition (when deleted, can release just enough storage).
        let (size, mut ans) = n
            .children
            .values()
            .map(|c| Self::traverse2(c.clone(), target))
            .fold((0, u64::MAX), |acc, x| {
                (
                    acc.0 + x.0,
                    if x.1 < acc.1 && x.1 >= target {
                        x.1
                    } else {
                        acc.1
                    },
                )
            });
        if size < ans && size >= target {
            ans = size;
        }

        return (size, ans);
    }
}

impl Solution for Puzzle {
    type Output = u64;

    /// Solution for part 1.
    fn solve1(self) -> Result<Self::Output> {
        let (_, sum) = Self::traverse1(self.tree);
        Ok(sum)
    }

    /// Solution for part 2.
    fn solve2(self) -> Result<Self::Output> {
        let total_available = 70_000_000;
        let required_unused = 30_000_000;

        let current_unused = total_available - self.tree.borrow().size();
        let release_target = required_unused - current_unused;

        let (_, size) = Self::traverse2(self.tree, release_target);
        Ok(size)
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
            Puzzle::from_file(format!("testdata/day_07/{stage}/input.txt").as_str()).unwrap();
        let answer = match part {
            1 => puzzle.solve1(),
            2 => puzzle.solve2(),
            _ => unreachable!(),
        }
        .unwrap();

        let expected_path = format!("testdata/day_07/{stage}/output-part{part}.txt");
        let expected = fs::read_to_string(&expected_path)
            .wrap_err(format!(
                "Failed to read expected output from {expected_path}"
            ))
            .unwrap();
        assert_eq!(answer.to_string(), expected.trim());
    }
}
