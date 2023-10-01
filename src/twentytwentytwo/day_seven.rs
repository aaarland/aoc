use std::{
    cell::RefCell,
    collections::{BTreeMap, HashMap},
    fmt,
    rc::Rc,
};

use camino::Utf8PathBuf;
use indexmap::IndexMap;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    combinator::{all_consuming, map},
    sequence::{preceded, separated_pair},
    Finish, IResult,
};

use crate::solutions::Solution;

pub struct DaySeven;
impl Solution for DaySeven {
    fn solve(&self, lines: Vec<String>) -> () {
        solution(lines);
    }
}

fn parse_path(i: &str) -> IResult<&str, Utf8PathBuf> {
    map(
        take_while1(|c: char| "abcdefghijklmnopqrstuvwxyz/.".contains(c)),
        Into::into,
    )(i)
}

#[derive(Debug)]
struct Ls;

fn parse_ls(i: &str) -> IResult<&str, Ls> {
    map(tag("ls"), |_| Ls)(i)
}

#[derive(Debug)]
struct Cd(Utf8PathBuf);

fn parse_cd(i: &str) -> IResult<&str, Cd> {
    map(preceded(tag("cd "), parse_path), Cd)(i)
}

#[derive(Debug)]
enum Command {
    Ls,
    Cd(Utf8PathBuf),
}

impl From<Ls> for Command {
    fn from(_ls: Ls) -> Self {
        Command::Ls
    }
}

impl From<Cd> for Command {
    fn from(cd: Cd) -> Self {
        Command::Cd(cd.0)
    }
}

fn parse_command(i: &str) -> IResult<&str, Command> {
    let (i, _) = tag("$ ")(i)?;
    alt((map(parse_ls, Into::into), map(parse_cd, Into::into)))(i)
}

#[derive(Debug)]
enum Entry {
    Dir(Utf8PathBuf),
    File(u64, Utf8PathBuf),
}

fn parse_entry(i: &str) -> IResult<&str, Entry> {
    let parse_file = map(
        separated_pair(nom::character::complete::u64, tag(" "), parse_path),
        |(size, path)| Entry::File(size, path),
    );
    let parse_dir = map(preceded(tag("dir "), parse_path), Entry::Dir);
    alt((parse_file, parse_dir))(i)
}

#[derive(Debug)]
enum Line {
    Command(Command),
    Entry(Entry),
}

fn parse_line(i: &str) -> IResult<&str, Line> {
    alt((
        map(parse_command, Line::Command),
        map(parse_entry, Line::Entry),
    ))(i)
}

type NodeHandle = Rc<RefCell<Node>>;
#[derive(Default)]
struct Node {
    size: usize,
    parent: Option<NodeHandle>,
    children: IndexMap<Utf8PathBuf, NodeHandle>,
}

impl Node {
    fn is_dir(&self) -> bool {
        self.size == 0 && !self.children.is_empty()
    }

    fn total_size(&self) -> u64 {
        self.children
            .values()
            .map(|child| child.borrow().total_size())
            .sum::<u64>()
            + self.size as u64
    }
}
fn all_dirs(n: NodeHandle) -> Box<dyn Iterator<Item = NodeHandle>> {
    let children = n.borrow().children.values().cloned().collect::<Vec<_>>();
    Box::new(
        std::iter::once(n).chain(
            children
                .into_iter()
                .filter_map(|child| {
                    if child.borrow().is_dir() {
                        Some(all_dirs(child))
                    } else {
                        None
                    }
                })
                .flatten(),
        ),
    )
}
struct PrettyNode<'a>(&'a NodeHandle);

impl<'a> fmt::Debug for PrettyNode<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let this = self.0.borrow();
        if this.size == 0 {
            writeln!(f, "(dir)")?;
        } else {
            writeln!(f, "(file, size={})", this.size)?;
        }

        for (name, child) in &this.children {
            // not very efficient at all, but shrug
            for (index, line) in format!("{:?}", PrettyNode(child)).lines().enumerate() {
                if index == 0 {
                    writeln!(f, "{name} {line}")?;
                } else {
                    writeln!(f, "  {line}")?;
                }
            }
        }
        Ok(())
    }
}
fn solution(lines: Vec<String>) {
    let lines = lines
        .iter()
        .map(|line| all_consuming(parse_line)(line.as_str()).finish().unwrap().1);

    let root = Rc::new(RefCell::new(Node::default()));
    let mut node = root.clone();
    for line in lines {
        match line {
            Line::Command(cmd) => match cmd {
                Command::Ls => {}
                Command::Cd(path) => match path.as_str() {
                    "/" => {}
                    ".." => {
                        let parent = node.borrow().parent.clone().unwrap();
                        node = parent;
                    }
                    _ => {
                        let child = node.borrow_mut().children.entry(path).or_default().clone();
                        node = child;
                    }
                },
            },
            Line::Entry(entry) => match entry {
                Entry::Dir(dir) => {
                    let entry = node.borrow_mut().children.entry(dir).or_default().clone();
                    entry.borrow_mut().parent = Some(node.clone());
                }
                Entry::File(size, file) => {
                    let entry = node.borrow_mut().children.entry(file).or_default().clone();
                    entry.borrow_mut().parent = Some(node.clone());
                    entry.borrow_mut().size = size as usize;
                }
            },
        }
    }
    let total_space = 70_000_000_u64;
    let used_space = root.borrow().total_size();
    let free_space = total_space.checked_sub(dbg!(used_space)).unwrap();
    let size_needed = 30_000_000_u64;
    let minimum_space_to_free = size_needed.checked_sub(dbg!(free_space)).unwrap();

    let removed_dir_size = all_dirs(root)
        .map(|d| d.borrow().total_size())
        .filter(|&size| size >= minimum_space_to_free)
        .inspect(|s| {
            dbg!(s);
        })
        .min();

    dbg!(removed_dir_size);
}
