use std::{io, iter, mem};
use std::io::{BufRead, BufReader};
use aoc_2023::coordinate::grid::Grid;
use aoc_2023::graph::CycleInfo;

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

    // let one_spin = rotate_grid_clockwise(
    //     spin_forever(orig_grid.clone()).take(4).last().unwrap()
    // );
    // println!("After one spin:");
    // println!("{}", display_grid(&one_spin));

    let possible_cycle = CycleInfo::check_cycle(
        // Prevent infinite memory
        spin_forever(orig_grid).take(1000)
    );
    println!("Cycle? {:?}", possible_cycle.is_ok());
    let cycle_info: CycleInfo<Grid<u8>> = possible_cycle.unwrap();
    println!("{} {}", cycle_info.dist_to_cycle_start(), cycle_info.cycle().len());
    let idx_within_cycle = (3_999_999_999usize - cycle_info.dist_to_cycle_start())
        % cycle_info.cycle().len();
    let first_idx = idx_within_cycle + cycle_info.dist_to_cycle_start();
    // println!("{idx_within_cycle} {first_idx} {}", first_idx % 4);
    let mut grid = cycle_info.cycle()[idx_within_cycle].clone();
    let needed_rotations = 4 - (first_idx % 4);
    assert_eq!(needed_rotations, 1);
    grid = grid.rotate_clockwise();

    // for (check_idx, mut check_grid) in cycle_info.cycle().iter().cloned().enumerate() {
    //     check_grid = rotate_grid_clockwise(check_grid);
    //     let total_load = north_load(&check_grid);
    //     println!("{check_idx} -> {total_load}");
    // }

    north_load(&grid)
}

fn spin_forever(mut grid: Grid<u8>) -> impl Iterator<Item=Grid<u8>> {
    iter::from_fn(move || {
        roll_boulders(&mut grid);
        let clone = grid.clone();
        // Can't move grid directly because it's captured by a closure
        // This became less performant after improved grid...
        let mut tmp = Grid::full(1, 1, 0);
        mem::swap(&mut tmp, &mut grid);
        // I guess the idea is this method could panic and that panic could be caught
        grid = tmp.rotate_clockwise();
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
    for row_idx in (0..grid.rows()).rev() {
        match grid[row_idx][col_idx] as char {
            '.' => (),
            'O' => {
                boulder_count += 1;
                grid[row_idx][col_idx] = '.' as u8;
            },
            '#' => {
                for backfill_idx in (row_idx + 1)..(row_idx + 1 + boulder_count) {
                    grid[backfill_idx][col_idx] = 'O' as u8;
                }
                boulder_count = 0;
            },
            _ => panic!("Invalid character: {:?}", grid[row_idx][col_idx]),
        }
    }
    for backfill_idx in 0..boulder_count {
        grid[backfill_idx][col_idx] = 'O' as u8;
    }
}

fn north_load(grid: &Grid<u8>) -> usize {
    let mut load = 0usize;
    for (idx, row) in grid.iter_rows().enumerate() {
        for c in row.iter().copied() {
            match c as char {
                'O' => load += grid.rows() - idx,
                _ => (),
            }
        }
    }
    load
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_north_load() {
        let grid: Grid<u8> = "\
OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#....".lines()
            .map(|s| s.as_bytes().to_vec())
            .collect();
        assert_eq!(north_load(&grid), 136);
    }
}
