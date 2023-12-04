use std::collections::HashMap;
use std::io;
use std::io::{BufRead, BufReader};
use regex::Regex;

fn process_lines(lines: impl Iterator<Item=String>) -> u32 {
    let re = Regex::new(r"(\d+) (red|green|blue)").unwrap();
    let maxes: HashMap<String, u32> = [
        ("red", 12),
        ("green", 13),
        ("blue", 14),
    ].iter().map(|&(s, n)| (String::from(s), n)).collect();
    lines
        .enumerate()
        .filter_map(|(idx, s)| {
            let idx: u32 = idx.try_into().expect("overflow");
            for m in re.captures_iter(&s) {
                let (_, [num, color]) = m.extract();
                let num: u32 = num.parse().expect(&format!("Could not parse {num:?}"));
                if num > *maxes.get(color).unwrap() {
                    return None;
                }
            }
            Some(idx + 1)
        })
        .sum::<u32>()
}

fn main() {
    let stdin = io::stdin();
    let reader = BufReader::with_capacity(256, stdin.lock());
    println!("{}", process_lines(reader.lines()
        .map(|s| s.expect("unicode issue"))))
}