use std::collections::HashSet;
use std::io;
use std::io::{BufRead, BufReader};
use regex::Regex;

fn parse_number(s: &str) -> u32 {
    s.parse().expect(&format!("number parse issue {s:?}"))
}

fn process_lines(lines: impl Iterator<Item=String>) -> u32 {
    let re = Regex::new(r"\d+").unwrap();
    let mut counts: Vec<u32> = vec![];
    let mut cards_seen = 0usize;
    lines
        .map(|line| {
            cards_seen += 1;
            let (_, rest) = line.split_once(':').expect("Could not find :");
            let (winning, have) = rest.split_once('|').expect("Could not find |");
            let winning: HashSet<_> = re.find_iter(winning)
                .map(|m| parse_number(m.as_str()))
                .collect();
            let have: HashSet<_> = re.find_iter(have)
                .map(|m| parse_number(m.as_str()))
                .collect();
            have.intersection(&winning).count()
        })
        .enumerate()
        .for_each(|(idx, winners)| {
            let largest_card = idx + winners;
            // It would be nice to have a method like truncate that only grows a vector
            counts.resize(counts.len().max(largest_card + 1), 0);
            counts[idx] += 1;
            // println!("Have an original {}; total {}", idx + 1, counts[idx]);
            for card in (idx + 1)..=largest_card {
                // print!("Got {} copy/ies of {} from {}", counts[idx], card + 1, idx + 1);
                counts[card] += counts[idx];
                // println!("; total {}", counts[card])
            }
        });
    // We have potentially added 'copies' of cards that we do not have originals of; get rid of
    // these.
    counts.truncate(cards_seen);
    // println!("{counts:?}");
    counts.iter().sum::<u32>()
}

fn main() {
    let stdin = io::stdin();
    let reader = BufReader::with_capacity(256, stdin.lock());
    println!("{}", process_lines(reader.lines()
        .map(|s| s.expect("unicode issue"))))
}
