use std::collections::HashSet;
use std::io;
use std::io::{BufRead, BufReader};
use regex::Regex;

fn parse_number(s: &str) -> u32 {
    s.parse().expect(&format!("number parse issue {s:?}"))
}

fn process_lines(lines: impl Iterator<Item=String>) -> u32 {
    let re = Regex::new(r"\d+").unwrap();
    lines
        .map(|line| {
            let (_, rest) = line.split_once(':').expect("Could not find :");
            let (winning, have) = rest.split_once('|').expect("Could not find |");
            let winning: HashSet<_> = re.find_iter(winning)
                .map(|m| parse_number(m.as_str()))
                .collect();
            let have: HashSet<_> = re.find_iter(have)
                .map(|m| parse_number(m.as_str()))
                .collect();
            let winners = have.intersection(&winning).count();
            if winners == 0 {
                0u32
            } else {
                (1 << (winners - 1)) as u32
            }
        })
        .sum::<u32>()
}

fn main() {
    let stdin = io::stdin();
    let reader = BufReader::with_capacity(256, stdin.lock());
    println!("{}", process_lines(reader.lines()
        .map(|s| s.expect("unicode issue"))))
}
