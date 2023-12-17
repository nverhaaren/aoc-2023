use std::{io};
use std::collections::HashSet;
use std::io::{BufRead, BufReader};
use itertools::Itertools;

use aoc_2023::graph::taxicab_dist_u;

fn process_lines(lines: impl Iterator<Item=String>) -> usize {
    let coordinates = get_coordinates(lines);
    let galaxy_rows: HashSet<usize> = coordinates.iter().copied()
        .map(|(r, _c)| r)
        .collect();
    let galaxy_cols: HashSet<usize> = coordinates.iter().copied()
        .map(|(_r, c)| c)
        .collect();
    let row_map = idx_map(&galaxy_rows);
    let col_map = idx_map(&galaxy_cols);

    coordinates.iter().copied().cartesian_product(coordinates.iter().copied())
        .map(|(c1, c2)| {
            ((row_map[c1.0], col_map[c1.1]), (row_map[c2.0], col_map[c2.1]))
        })
        .map(|(c1, c2)| {
            taxicab_dist_u(c1, c2)
        })
        .sum::<usize>() / 2  // double counts
}

fn get_coordinates(lines: impl Iterator<Item=String>) -> Vec<(usize, usize)> {
    lines
        .enumerate()
        .flat_map(|(row, line)| -> Vec<_> {
            line.chars()
                .enumerate()
                .filter_map(move |(col, c)| {
                    if c == '#' {
                        Some((row, col))
                    } else {
                        None
                    }
                })
                .collect()  // What is the nice way to deal with this?
        }).collect()
}

fn idx_map(galaxy_idxs: &HashSet<usize>) -> Vec<usize> {
    let max: usize = galaxy_idxs.iter().copied().max().expect("No galaxies");
    let mut shift = 0usize;
    (0..=max).into_iter()
        .map(|idx| {
            if !galaxy_idxs.contains(&idx) {
                shift += 1;
            }
            idx + shift
        })
        .collect()
}

fn main() {
    let stdin = io::stdin();
    let reader = BufReader::with_capacity(256, stdin.lock());
    println!("{}", process_lines(reader.lines()
        .map(|s| s.expect("unicode issue"))))
}
