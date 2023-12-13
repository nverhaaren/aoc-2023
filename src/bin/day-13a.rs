use std::{io, iter};
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use itertools::Itertools;

fn main() {
    let stdin = io::stdin();
    let reader = BufReader::with_capacity(256, stdin.lock());
    println!("{}", process_lines(reader.lines()
        .map(|s| s.expect("unicode issue"))))
}

type Mirror = Vec<Vec<u8>>;

fn process_lines(mut lines: impl Iterator<Item=String>) -> usize {
    iter::from_fn(move || load_mirror(&mut lines))
        .map(|mirror| -> usize {
            todo!()
        })
        .sum()

}

fn row_could_reflect(row: &[u8], idx: usize) -> bool {
    for row_idx in 0..row.len() {
        let other_idx = if row_idx <= idx {
            idx + 1 + (idx - row_idx)
        } else {
            match idx.checked_sub(row_idx - (idx + 1)) {
                Some(x) => x,
                None => continue,
            }
        };
        if other_idx >= row.len() {
            continue
        }
        if row[row_idx] != row[other_idx] {
            return false
        }
    }
    true
}

fn load_mirror(mut lines: impl Iterator<Item=String>) -> Option<Mirror> {
    let result: Vec<_> = lines
        .take_while(|line| !line.is_empty())
        .map(|line| line.into_bytes())
        .collect();
    if !result.is_empty() {
        Some(result)
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_row_could_reflect() {
        let row = "#.##..##.".as_bytes();
        assert_eq!(
            (0..(row.len() - 1)).into_iter()
                .filter(|idx| row_could_reflect(row, *idx))
                .collect::<Vec<_>>(),
            vec![4, 6]
        );
    }
}
