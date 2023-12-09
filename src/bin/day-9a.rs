use std::collections::{HashMap, HashSet};
use std::{io, mem};
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use anyhow::anyhow;
use itertools::Itertools;
use regex::Regex;
use aoc_2023::util::parse_signed_number;

fn extrapolate_history(mut history: Vec<i64>) -> i64 {
    let mut lasts = vec![];
    let mut diff = vec![];
    while !history.iter().copied().all(|x| x == 0) {
        lasts.push(history.last().copied().unwrap());
        diff.extend(
        history.iter().copied().tuple_windows()
                .map(|(a, b): (i64, i64)| {
                    b - a
                })
        );
        mem::swap(&mut history, &mut diff);
        diff.clear();
    }
    lasts.iter().sum()
}

fn process_lines(lines: impl Iterator<Item=String>) -> i64 {
    let re = Regex::new(r"-?\d+").unwrap();
    lines
        .map(|line| -> Vec<i64> {
            re.find_iter(&line)
                .map(|s| parse_signed_number(s.as_str()))
                .collect()
        })
        .map(extrapolate_history)
        .sum()
}

fn main() {
    let stdin = io::stdin();
    let reader = BufReader::with_capacity(256, stdin.lock());
    println!("{}", process_lines(reader.lines()
        .map(|s| s.expect("unicode issue"))))
}
