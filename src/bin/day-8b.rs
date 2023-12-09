use std::collections::{HashMap, HashSet};
use std::io;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use anyhow::anyhow;
use regex::Regex;
use aoc_2023::check_cycle;

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

#[allow(unused)]
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

fn wander<'a>(directions: &'a[Direction], map: &'a HashMap<Node, (Node, Node)>, start: Node) -> impl Iterator<Item=(Node, usize)> + 'a {
    let mut current = start;
    directions.iter().copied().enumerate().cycle()
        .map(move |(idx, d)| {
            let result = (current, idx);
            current = match d {
                Direction::Left => map.get(&current).unwrap().0,
                Direction::Right => map.get(&current).unwrap().1,
            };
            result
        })
}

fn process_lines(lines: impl Iterator<Item=String>) -> u64 {
    // checked first billion or so naively without success
    let (directions, map) = load(lines);

    let starts: Vec<Node> = map.keys().copied().filter(|n| n.last_a()).collect();
    let _cycles: Vec<_> = starts.iter().copied()
        .map(|node| {
            check_cycle(wander(&directions, &map, node)).expect("No cycle found")
        })
        .inspect(|c| {
            println!(
                "Dist to cycle: {}, cycle len: {}, possible endpoints: {}, first endpoint idx: {:?}",
                c.dist_to_cycle_start(),
                c.cycle().len(),
                c.cycle().iter().filter(|(n, _)| n.last_z()).count(),
                c.cycle().iter().enumerate().filter(|(_, (n, _))| n.last_z()).next()
            )
        })
        .collect();

    // Output:

    // Dist to cycle: 2, cycle len: 21251, possible endpoints: 1, first endpoint idx: Some((21249, (Node([72, 82, 90]), 0)))
    // Dist to cycle: 2, cycle len: 19099, possible endpoints: 1, first endpoint idx: Some((19097, (Node([84, 77, 90]), 0)))
    // Dist to cycle: 2, cycle len: 12643, possible endpoints: 1, first endpoint idx: Some((12641, (Node([68, 71, 90]), 0)))
    // Dist to cycle: 2, cycle len: 11567, possible endpoints: 1, first endpoint idx: Some((11565, (Node([90, 90, 90]), 0)))
    // Dist to cycle: 2, cycle len: 14257, possible endpoints: 1, first endpoint idx: Some((14255, (Node([70, 66, 90]), 0)))
    // Dist to cycle: 4, cycle len: 16409, possible endpoints: 1, first endpoint idx: Some((16405, (Node([74, 86, 90]), 0)))
    // 0

    // Very conveniently for all of these we land on the one endpoint after the first period of the
    // cycle, so we land on the endpoint iff we have taken a number of steps that is a multiple of
    // the period of the cycle. So we just need to lcm the periods.

    // So I didn't need all the Chinese Remainder stuff anyway. It may be useful in the future
    // though.
    0
}

fn main() {
    let stdin = io::stdin();
    let reader = BufReader::with_capacity(256, stdin.lock());
    println!("{}", process_lines(reader.lines()
        .map(|s| s.expect("unicode issue"))))
}
