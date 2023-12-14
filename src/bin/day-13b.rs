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
            // println!("New mirror");
            let mut result = 0usize;
            if let Some(idx) = vertical_reflection(&mirror) {
                // println!("Found vertical: {idx}");
                result += idx + 1;
            }
            if let Some(idx) = horizontal_reflection(&mirror) {
                // println!("Found horizontal: {idx}");
                result += (idx + 1) * 100;
            }
            result
        })
        .sum()

}

fn vertical_reflection(mirror: &Mirror) -> Option<usize> {
    // Assumes that there are not multiple solutions
    let mut candidate_counts: Vec<_> = vec![0usize; mirror[0].len() - 1];
    for row in mirror.iter() {
        (0..(mirror[0].len() - 1)).into_iter()
            .filter(|x| {
                row_could_reflect(row.as_slice(), *x)
            })
            .for_each(|x| {
                candidate_counts[x] += 1;
            });
    }
    candidate_counts.iter().copied()
        .enumerate()
        .filter(|(idx_, count)| *count == mirror.len() - 1)
        .map(|(idx, count_)| idx)
        .next()
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

fn horizontal_reflection(mirror: &Mirror) -> Option<usize> {
    // Assumes that there are not multiple solutions
    let mut candidate_counts: Vec<_> = vec![0usize; mirror.len() - 1];
    for idx_of_col in 0..(mirror[0].len()) {
        (0..(mirror.len() - 1)).into_iter()
            .filter(|x| {
                col_could_reflect(mirror, idx_of_col, *x)
            })
            .for_each(|x| {
                candidate_counts[x] += 1;
            });
    }
    candidate_counts.iter().copied()
        .enumerate()
        .filter(|(idx_, count)| *count == mirror[0].len() - 1)
        .map(|(idx, count_)| idx)
        .next()
}

fn col_could_reflect(mirror: &Mirror, idx_of_col: usize, idx: usize) -> bool {
    for col_idx in 0..mirror.len() {
        let other_idx = if col_idx <= idx {
            idx + 1 + (idx - col_idx)
        } else {
            match idx.checked_sub(col_idx - (idx + 1)) {
                Some(x) => x,
                None => continue,
            }
        };
        if other_idx >= mirror.len() {
            continue
        }
        if mirror[col_idx][idx_of_col] != mirror[other_idx][idx_of_col] {
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

    #[test]
    fn test_col_could_reflect() {
        let mirror_str = "\
#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
";
        let mirror = load_mirror(
            mirror_str.lines().map(|s| s.to_owned())
        ).unwrap();
        assert!(col_could_reflect(&mirror, 8, 0));
        assert!(col_could_reflect(&mirror, 8, 3));
    }
}
