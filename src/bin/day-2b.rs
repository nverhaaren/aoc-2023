use std::collections::HashMap;
use std::io;
use std::io::{BufRead, BufReader};
use regex::Regex;

fn process_lines(lines: impl Iterator<Item=String>) -> u32 {
    let re = Regex::new(r"(\d+) (red|green|blue)").unwrap();
    lines
        .map(|s| -> u32 {
            let mut mins: HashMap<String, u32> = [
                ("red", 0),
                ("green", 0),
                ("blue", 0),
            ].iter().map(|&(s, n)| (String::from(s), n)).collect();
            for capture in re.captures_iter(&s) {
                let (_, [count, color]) = capture.extract();
                let count: u32 = count.parse().expect("Number parse issue");
                let min = mins.get_mut(color).unwrap();
                *min = (*min).max(count);
            }
            mins.values().product::<u32>()
        })
        .sum::<u32>()
}

fn main() {
    let stdin = io::stdin();
    let reader = BufReader::with_capacity(256, stdin.lock());
    println!("{}", process_lines(reader.lines()
        .map(|s| s.expect("unicode issue"))))
}