use std::{io, iter, mem};
use std::io::{BufRead, BufReader};
use aoc_2023::coordinate::{Grid, rotate_grid_clockwise};
use aoc_2023::graph::CycleInfo;

fn main() {
    let stdin = io::stdin();
    let reader = BufReader::with_capacity(256, stdin.lock());
    println!("{}", process_lines(reader.lines()
        .map(|s| s.expect("unicode issue"))))
}

fn process_lines(mut lines: impl Iterator<Item=String>) -> usize {
    let orig_grid: Vec<_> = lines
        .map(|line| line.into_bytes())
        .collect();
    let possible_cycle = CycleInfo::check_cycle(
        // Prevent infinite memory
        spin_forever(orig_grid).take(1000)
    );
    println!("{:?}", possible_cycle.is_some());
    possible_cycle.map(|cycle_info| {
        println!("{} {}", cycle_info.dist_to_cycle_start(), cycle_info.cycle().len());
    });
    0
}

fn spin_forever(mut grid: Grid<u8>) -> impl Iterator<Item=Grid<u8>> {
    iter::from_fn(move || {
        roll_boulders(&mut grid);
        let clone = grid.clone();
        // Can't move grid directly because it's captured by a closure
        let mut tmp = vec![];
        mem::swap(&mut tmp, &mut grid);
        // I guess the idea is this method could panic and that panic could be caught
        grid = rotate_grid_clockwise(tmp);
        Some(clone)
    })
}

fn roll_boulders(grid: &mut Grid<u8>) {
    for col_idx in 0..grid[0].len() {
        roll_boulders_column(grid, col_idx);
    }
}

fn roll_boulders_column(grid: &mut Grid<u8>, col_idx: usize) {
    let mut boulder_count = 0usize;
    for row_idx in (0..grid.len()).rev() {
        match grid[row_idx][col_idx] as char {
            '.' => (),
            'O' => {
                boulder_count += 1;
                grid[row_idx][col_idx] = '.' as u8;
            },
            '#' => {
                for backfill_idx in row_idx..(row_idx + boulder_count) {
                    grid[backfill_idx][col_idx] = 'O' as u8;
                }
                boulder_count = 0;
            },
            _ => panic!("Invalid character: {:?}", grid[row_idx][col_idx]),
        }
    }
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
        round_count = 0;
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
