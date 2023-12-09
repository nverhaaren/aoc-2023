use std::collections::HashMap;
use std::io;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use anyhow::anyhow;
use regex::Regex;

// should really move this to lib
#[allow(dead_code)]
fn parse_number(s: &str) -> u64 {
    s.parse().expect(&format!("number parse issue {s:?}"))
}

trait InspectVal: Sized {
    fn inspect_val(self, f: impl FnOnce(&Self) -> ()) -> Self {
        f(&self);
        self
    }

    fn inspect_val_mut(mut self, f: impl FnOnce(&mut Self) -> ()) -> Self {
        f(&mut self);
        self
    }
}

impl<T> InspectVal for T {}

////////////

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Direction {
    Left,
    Right,
}

impl TryFrom<char> for Direction {
    type Error = char;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(Direction::Left),
            'R' => Ok(Direction::Right),
            _ => Err(value),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Node([u8; 3]);

impl FromStr for Node {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        <[u8; 3]>::try_from(s.as_bytes())
            .map_err(|e| anyhow!("Node string was wrong length: {e}"))
            .map(|a| Self(a))
    }
}

fn process_lines(mut lines: impl Iterator<Item=String>) -> u64 {
    let re = Regex::new(r"([A-Z]{3}) = \(([A-Z]{3}), ([A-Z]{3})\)").unwrap();

    let first = lines.next().expect("empty input");
    assert!(lines.next().expect("early termination").is_empty());

    let directions: Vec<Direction> = first.chars()
        .map(|c| Direction::try_from(c))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|c| format!("Got invalid direction {c}")).unwrap();

    let map: HashMap<Node, (Node, Node)> = lines
        .map(|line| -> (Node, (Node, Node)) {
            let (_, [source, left, right]) = re.captures(&line)
                .expect("Unexpected line format").extract();
            (source.parse().unwrap(), (left.parse().unwrap(), right.parse().unwrap()))
        })
        .collect();

    let mut current: Node = "AAA".parse().unwrap();
    let zzz: Node = "ZZZ".parse().unwrap();
    directions.iter().copied()
        .cycle()
        .take_while(|d| {
            let node = match *d {
                Direction::Left => map.get(&current).copied().unwrap().0,
                Direction::Right => map.get(&current).copied().unwrap().1,
            };
            let result = node != zzz;
            current = node;
            result
        })
        .count() as u64 + 1  // + 1 because we don't take the final step
}

fn main() {
    let stdin = io::stdin();
    let reader = BufReader::with_capacity(256, stdin.lock());
    println!("{}", process_lines(reader.lines()
        .map(|s| s.expect("unicode issue"))))
}
