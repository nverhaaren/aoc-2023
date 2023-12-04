use std::io;
use std::io::{BufRead, BufReader};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = BufReader::with_capacity(256, stdin.lock());
    let calibration: io::Result<Vec<u32>> = reader.lines()
        .map(|lr| lr.map(|l| {
            let mut val: Option<(char, char)> = None;
            for c in l.chars().filter(|c| c.is_numeric()) {
                val = Some(match val {
                    None => (c, c),
                    Some((i, _)) => (i, c),
                });
            }
            let (first, last) = val.expect(format!("Did not find digit in {l:?}").as_str());
            10 * first.to_digit(10).unwrap() + last.to_digit(10).unwrap()
        }))
        .collect();
    calibration.map(|v| println!("{}", v.iter().sum::<u32>()))
}
