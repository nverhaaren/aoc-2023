use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::io;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use anyhow::anyhow;
use itertools::Itertools;
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

impl Node {
    pub fn last_a(&self) -> bool {
        self.0[2] == ('A' as u8)
    }

    pub fn last_z(&self) -> bool {
        self.0[2] == ('Z' as u8)
    }
}

impl FromStr for Node {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        <[u8; 3]>::try_from(s.as_bytes())
            .map_err(|e| anyhow!("Node string was wrong length: {e}"))
            .map(|a| Self(a))
    }
}

fn load(mut lines: impl Iterator<Item=String>) -> (Vec<Direction>, HashMap<Node, (Node, Node)>) {
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
    (directions, map)
}

fn analyze(lines: impl Iterator<Item=String>) {
    let (directions, map) = load(lines);

    let end_in_a: Vec<Node> = map.keys()
        .filter(|n| n.last_a())
        .copied()
        .collect();
    let end_in_z: Vec<Node> = map.keys()
        .filter(|n| n.last_z())
        .copied()
        .collect();

    let cycles: Vec<_> = end_in_a.iter().copied()
        .map(|node| {
            find_cycle(&directions, &map, node)
        })
        .collect();

    let cycle_lens: Vec<_> = cycles.iter()
        .map(|set| set.len())
        .collect();

    let starts_in_cycles: Vec<_> = cycles.iter()
        .map(|set| {
            set.iter().filter(|n| n.last_a()).count()
        })
        .collect();

    println!("end_in_a: {}, end_in_z: {}", end_in_a.len(), end_in_z.len());
    println!("Lengths: {cycle_lens:?}");
    println!("Duplicates? {starts_in_cycles:?}");
}

fn find_cycle(directions: &[Direction], map: &HashMap<Node, (Node, Node)>, start: Node) -> HashSet<Node>  {
    let mut current = start;
    let mut result = HashSet::new();
    result.insert(start);
    directions.iter().copied().cycle()
        .take_while(|d| {
            current = match d {
                Direction::Left => map.get(&current).unwrap().0,
                Direction::Right => map.get(&current).unwrap().1,
            };
            result.insert(current)
        })
        .for_each(|_| {});
    result
}

fn process_lines(mut lines: impl Iterator<Item=String>) -> u64 {
    let (directions, map) = load(lines);

    let mut currents: Vec<Node> = map.keys().copied().filter(|n| n.last_a()).collect();
    directions.iter().copied()
        .cycle()
        .take_while(|d| {
            let next_nodes: Vec<Node> = currents.iter().copied().map(|node| {
                match *d {
                    Direction::Left => map.get(&node).copied().unwrap().0,
                    Direction::Right => map.get(&node).copied().unwrap().1,
                }
            }).collect();
            currents = next_nodes;
            !currents.iter().all(|node| node.last_z())
        })
        .enumerate()
        .inspect(|(idx, _)| {
            if idx % 1_000_000 == 0 {
                println!("Found {idx}");
            }
        })
        .count() as u64 + 1  // + 1 because we don't take the final step
}

fn main() {
    let stdin = io::stdin();
    let reader = BufReader::with_capacity(256, stdin.lock());
    println!("{}", process_lines(reader.lines()
        .map(|s| s.expect("unicode issue"))))
}
