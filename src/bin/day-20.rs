use std::{io, str};
use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use anyhow::{anyhow, bail};
use itertools::Itertools;

fn main() {
    let stdin = io::stdin();
    let reader = BufReader::with_capacity(256, stdin.lock());
    let lines: Vec<_> = reader.lines()
        .try_collect().expect("Unicode issue");

    // println!("First part: {result} ({elapsed:.2?})");
    // println!("Second part: {}", part_2(&map));
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
enum ModuleKind {
    Broadcaster,
    FlipFlop,
    Conjunction,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
struct ModuleSpec {
    name: String,
    kind: ModuleKind,
    destinations: Vec<String>,
}

impl FromStr for ModuleSpec {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(' ');
        let core = split.next().ok_or(anyhow!("empty input"))?;
        if split.next() != Some("->") {
            bail!("did not find \"->\"");
        }
        let mut destinations = vec![];
        for s in split {
            let name = s.strip_suffix(',').unwrap_or(s).to_owned();
            destinations.push(name);
        }
        match core.strip_prefix('%') {
            None => (),
            Some(name) => return Ok(Self {
                name: name.to_owned(), kind: ModuleKind::FlipFlop, destinations
            }),
        };
        match core.strip_prefix('&') {
            None => (),
            Some(name) => return Ok(Self {
                name: name.to_owned(), kind: ModuleKind::Conjunction, destinations
            }),
        };
        if core != "broadcaster" {
            bail!("Unrecognized core: {core:?}");
        }
        Ok(Self { name: core.to_owned(), kind: ModuleKind::Broadcaster, destinations})
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_module_spec_a() {
        let ms: ModuleSpec = "broadcaster -> a, b, c".parse().unwrap();
        assert_eq!(ModuleSpec {
            name: "broadcaster".to_owned(),
            kind: ModuleKind::Broadcaster,
            destinations: ["a", "b", "c"].iter().copied().map(|s| s.to_owned()).collect(),
        }, ms);
    }

    #[test]
    fn test_parse_module_spec_b() {
        let ms: ModuleSpec = "%c -> inv".parse().unwrap();
        assert_eq!(ModuleSpec {
            name: "c".to_owned(),
            kind: ModuleKind::FlipFlop,
            destinations: ["inv"].iter().copied().map(|s| s.to_owned()).collect(),
        }, ms);
    }

    #[test]
    fn test_parse_module_spec_c() {
        let ms: ModuleSpec = "&inv -> a".parse().unwrap();
        assert_eq!(ModuleSpec {
            name: "inv".to_owned(),
            kind: ModuleKind::Conjunction,
            destinations: ["a"].iter().copied().map(|s| s.to_owned()).collect(),
        }, ms);
    }
}
