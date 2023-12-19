use std::{io, str};
use std::io::{BufRead, BufReader};
use std::ops::Index;
use std::str::FromStr;
use itertools::Itertools;
use regex::Regex;
use aoc_2023::util::parse_number;

fn main() {
    let stdin = io::stdin();
    let reader = BufReader::with_capacity(256, stdin.lock());
    let lines: Vec<_> = reader.lines()
        .try_collect().expect("Unicode issue");
    // println!("First part: {}", compute_covered(&plans_1));
    // println!("Second part: {}", compute_covered(&plans_2));
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
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

    pub fn guard_dest_re(&self, s: &str) -> Result<(Guard, Destination), Destination> {
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
}

// todo: full line parsing (rules then parts)

#[cfg(test)]
mod test {
    use super::*;

}
