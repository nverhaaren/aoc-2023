use std::{io, str};
use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::ops::Index;
use std::str::FromStr;
use std::time::Instant;
use itertools::Itertools;
use regex::Regex;
use aoc_2023::util::parse_number;

fn main() {
    let stdin = io::stdin();
    let reader = BufReader::with_capacity(256, stdin.lock());
    let lines: Vec<_> = reader.lines()
        .try_collect().expect("Unicode issue");
    let mut lines_iter = lines.iter();
    let parser = Parser::new();
    let map = parse_rules((&mut lines_iter).map(|s| s.as_str()), &parser)
        .collect();
    let parts: Vec<_> = parse_parts(lines_iter.map(|s| s.as_str()), &parser).collect();
    let now = Instant::now();
    let result = part_1(&map, parts.into_iter());
    let elapsed = now.elapsed();
    println!("First part: {result} ({elapsed:.2?})");
    println!("Second part: {}", part_2(&map));
}

fn part_1(map: &HashMap<String, Rule>, parts: impl Iterator<Item=Part>) -> u64 {
    parts
        .filter(|part| {
            let mut next = "in";
            loop {
                let rule = map.get(next).expect("map issue");
                match rule.destination(part) {
                    Destination::Accept => break true,
                    Destination::Reject => break false,
                    Destination::Rule(s) => next = s.as_str(),
                }
            }
        })
        .map(|part| part.total_rating())
        .sum()
}

fn part_2(map: &HashMap<String, Rule>) -> u64 {
    let mut x_boundaries = vec![1u64, 4001];
    let mut m_boundaries = vec![1u64, 4001];
    let mut a_boundaries = vec![1u64, 4001];
    let mut s_boundaries = vec![1u64, 4001];

    for rule in map.values() {
        for (guard, _) in &rule.chain {
            let value = if guard.less_than {
                guard.value
            } else {
                guard.value + 1
            };
            match guard.field {
                Field::X => x_boundaries.push(value),
                Field::M => m_boundaries.push(value),
                Field::A => a_boundaries.push(value),
                Field::S => s_boundaries.push(value),
            }
        }
    }

    x_boundaries.sort();
    m_boundaries.sort();
    a_boundaries.sort();
    s_boundaries.sort();

    let x_slices = boundaries_to_slices(x_boundaries);
    let m_slices = boundaries_to_slices(m_boundaries);
    let a_slices = boundaries_to_slices(a_boundaries);
    let s_slices = boundaries_to_slices(s_boundaries);

    let mut total = 0;
    for v in [x_slices, m_slices, a_slices, s_slices].iter().multi_cartesian_product() {
        let part = Part { x: v[0].0, m: v[1].0, a: v[2].0, s: v[3].0 };
        let mut next = "in";
        if loop {
            let rule = map.get(next).expect("map issue");
            match rule.destination(&part) {
                Destination::Accept => break true,
                Destination::Reject => break false,
                Destination::Rule(s) => next = s.as_str(),
            }
        } {
            total += v[0].1 * v[1].1 * v[2].1 * v[3].1;
        }
    }

    total
}

fn boundaries_to_slices(v: Vec<u64>) -> Vec<(u64, u64)> {
    let mut result = vec![];
    result.reserve_exact(v.len() - 1);
    for (low, high) in v.into_iter().tuple_windows() {
        result.push((low, high - low));
    }
    result
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

impl Part {
    pub fn total_rating(&self) -> u64 {
        self.x + self.m + self.a + self.s
    }
}

impl Index<Field> for Part {
    type Output = u64;
    fn index(&self, index: Field) -> &Self::Output {
        match index {
            Field::X => &self.x,
            Field::M => &self.m,
            Field::A => &self.a,
            Field::S => &self.s,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
enum Field { X, M, A, S }

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum Destination {
    Rule(String),
    Accept,
    Reject,
}

impl FromStr for Destination {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "A" => Destination::Accept,
            "R" => Destination::Reject,
            _ => Destination::Rule(s.to_owned())
        })
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
struct Guard {
    field: Field,
    less_than: bool,
    value: u64,
}

impl Guard {
    pub fn matches(&self, part: &Part) -> bool {
        let part_value = part[self.field];
        if self.less_than {
            part_value < self.value
        } else {
            part_value > self.value
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
struct Rule {
    chain: Vec<(Guard, Destination)>,
    terminal: Destination,
}

impl Rule {
    pub fn destination<'a>(&'a self, part: &Part) -> &'a Destination {
        self.chain.iter()
            .filter_map(|(guard, dest)| {
                if guard.matches(part) {
                    Some(dest)
                } else {
                    None
                }
            })
            .next()
            .unwrap_or(&self.terminal)
    }
}

// Parsing

#[derive(Debug, Clone)]
struct Parser {
    part_re: Regex,
    guard_dest_re: Regex,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            part_re: Regex::new(r"\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)}").unwrap(),
            guard_dest_re: Regex::new(r"([xmas])([<>])(\d+):([a-z]+|A|R)").unwrap(),
        }
    }

    pub fn parse_part(&self, s: &str) -> Part {
        let (_, [x, m, a, s]) = self.part_re.captures(s)
            .expect("Invalid part").extract();
        Part { x: parse_number(x), m: parse_number(m), a: parse_number(a), s: parse_number(s) }
    }

    pub fn parse_rule_entry(&self, s: &str) -> Result<(Guard, Destination), Destination> {
        let (_, [field, comp, value, dest]) = self.guard_dest_re.captures(s)
            .ok_or_else(|| s.parse().unwrap())?.extract();
        let field = match field {
            "x" => Field::X,
            "m" => Field::M,
            "a" => Field::A,
            "s" => Field::S,
            _ => panic!("Unknown field {field:?}"),
        };
        Ok((
            Guard { field, less_than: match comp { "<" => true, ">" => false, _ => panic!() }, value:parse_number(value) },
            dest.parse().unwrap()
        ))
    }

    pub fn parse_rule(&self, s: &str) -> (String, Rule) {
        let (before, after) = s.split_once('{').expect("Invalid rule");
        let (after, empty) = after.split_once('}').expect("Invalid rule");
        assert!(empty.is_empty());
        let mut chain = vec![];
        for entry in after.split(',') {
            match self.parse_rule_entry(entry) {
                Ok(parsed) => chain.push(parsed),
                Err(terminal) => {
                    return (before.to_owned(), Rule { chain, terminal })
                }
            }
        };
        unreachable!();
    }
}

fn parse_rules<'a>(
    it: impl Iterator<Item=&'a str> + 'a,
    parser: &'a Parser
) -> impl Iterator<Item=(String, Rule)> + 'a {
    it.take_while(|s| !s.is_empty())
        .map(move |s| parser.parse_rule(s))
}

fn parse_parts<'a>(
    it: impl Iterator<Item=&'a str> + 'a,
    parser: &'a Parser
) -> impl Iterator<Item=Part> + 'a {
    it.take_while(|s| !s.is_empty())
        .map(move |s| parser.parse_part(s))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_rule() {
        let expected = ("px".to_owned(), Rule { chain: vec![
            (
                Guard { field: Field::A, less_than: true, value: 2006 },
                Destination::Rule("qkq".to_owned())
            ),
            (
                Guard { field: Field::M, less_than: false, value: 2090 },
                Destination::Accept
            )
        ], terminal: Destination::Rule("rfg".to_owned()) });
        let parser = Parser::new();

        assert_eq!(expected, parser.parse_rule("px{a<2006:qkq,m>2090:A,rfg}"));
    }

    #[test]
    fn test_parse_part() {
        let part = Part { x: 787, m: 2655, a: 1222, s: 2876 };
        let parser = Parser::new();

        assert_eq!(part, parser.parse_part("{x=787,m=2655,a=1222,s=2876}"));
    }
}
