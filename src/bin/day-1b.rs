use std::collections::HashMap;
use std::io;
use std::io::{BufRead, BufReader};

use itertools::Itertools;
use regex::Regex;

#[allow(unused)]
fn first_last<T: Copy, I: Iterator<Item=T>>(it: I) -> Option<(T, T)> {
    let mut result = None;
    for t in it {
        result = Some(match result {
            None => (t, t),
            Some((i, _)) => (i, t),
        })
    }
    result
}

fn main() -> io::Result<()> {
    let mapping: HashMap<String, u32> = [
        ("one", 1),
        ("1", 1),
        ("two", 2),
        ("2", 2),
        ("three", 3),
        ("3", 3),
        ("four", 4),
        ("4", 4),
        ("five", 5),
        ("5", 5),
        ("six", 6),
        ("6", 6),
        ("seven", 7),
        ("7", 7),
        ("eight", 8),
        ("8", 8),
        ("nine", 9),
        ("9", 9),
    ].iter().map(|&(s, n)| (String::from(s), n)).collect();
    let back_mapping: HashMap<String, u32> = mapping.iter()
        .map(|(k, v)| (k.chars().rev().collect(), *v))
        .collect();
    // Collisions because of intersperse
    #[allow(unstable_name_collisions)]
    let pattern: String = mapping.keys().map(|s| s.as_str()).intersperse("|").collect();
    #[allow(unstable_name_collisions)]
    let back_pattern: String = back_mapping.keys().map(|s| s.as_str()).intersperse("|").collect();
    // eprintln!("{pattern}");
    // eprintln!("{back_pattern}");
    let re = Regex::new(&pattern).expect(&format!("Issue building pattern: {pattern:?}"));
    let back_re = Regex::new(&back_pattern).expect(&format!("Issue building pattern: {pattern:?}"));

    let stdin = io::stdin();
    let reader = BufReader::with_capacity(256, stdin.lock());
    let calibration: io::Result<Vec<u32>> = reader.lines()
        .map(|lr| lr.map(|l| {
            // let (first, last) = first_last(
            //     re.find_iter(&l).map(|m| m.as_str())
            // ).expect(&format!("No digit in {l:?}"));
            // 10 * mapping.get(first).unwrap() + mapping.get(last).unwrap()
            let front = mapping
                .get(re.find(&l)
                    .map(|m| m.as_str())
                    .expect(&format!("No digit in {l:?}")))
                .unwrap();
            let reversed: String = l.chars().rev().collect();
            let back = back_mapping
                .get(back_re.find(&reversed)
                    .map(|m| m.as_str())
                    .expect(&format!("No reversed digit in {reversed:?}")))
                .unwrap();
            10 * front + back
        }))
        .collect();
    calibration.map(|v| println!("{}", v.iter().sum::<u32>()))
}