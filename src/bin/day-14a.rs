use std::io;
use std::io::{BufRead, BufReader};
use aoc_2023::coordinate::grid::Grid;

fn main() {
    let stdin = io::stdin();
    let reader = BufReader::with_capacity(256, stdin.lock());
    println!("{}", process_lines(reader.lines()
        .map(|s| s.expect("unicode issue"))))
}

fn process_lines(lines: impl Iterator<Item=String>) -> usize {
    let orig_grid: Vec<_> = lines
        .map(|line| line.into_bytes())
        .collect();
    let orig_grid = Grid::try_from_vec_of_vecs(orig_grid).expect("Irregular input");
    let grid = orig_grid.transpose();
    grid.iter_rows()
        // .inspect(|_| println!("Line"))
        .map(row_load)
        .sum()
}

fn row_load(row: &[u8]) -> usize {
    let mut collection_points = vec![];
    let mut anchor = row.len();
    let mut round_count = 0usize;
    for (idx, c) in row.iter().copied().enumerate() {
        match c as char {
            'O' => round_count += 1,
            '.' => (),
            '#' => {
                // println!("Boulder at {idx} -> ({anchor} {round_count})");
                if round_count > 0 {
                    collection_points.push((anchor, round_count));
                    round_count = 0;
                }
                anchor = row.len() - idx - 1;
                // println!("New anchor {anchor}");
            },
            _ => panic!("Unknown character {c:?}"),
        }
    }
    if round_count > 0 {
        collection_points.push((anchor, round_count));
        // round_count = 0;
    }
    collection_points.into_iter()
        .map(|(anchor, count)| -> usize {
            (0usize..).into_iter()
                .map(|x| anchor - x)
                .take(count)
                .sum()
        })
        .sum()
}
