use std::{io, mem};
use std::collections::VecDeque;
use std::fmt::{Display, Formatter, Write};
use std::io::{BufRead, BufReader};
use itertools::Itertools;
use aoc_2023::coordinate::{Direction, UCoordinate};
use aoc_2023::coordinate::grid::Grid;
use aoc_2023::util::CheckedAdd;

fn main() {
    let stdin = io::stdin();
    let reader = BufReader::with_capacity(256, stdin.lock());
    let lines: Vec<_> = reader.lines()
        .map(|x| x.map(|s| s.into_bytes()))
        .try_collect().expect("Unicode issue");
    let grid = Grid::try_from_vec_of_vecs(lines).expect("Irregular input");
    println!("First part: {}", part_1(&grid));
    println!("Second part: {}", part_2(&grid));
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Default)]
struct Visited {
    row: bool,
    col: bool,
}

impl Display for Visited {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char(if self.visited() { '#' } else { '.' })
    }
}

#[allow(unused)]
impl Visited {
    pub const fn new() -> Self {
        Self { row: false, col: false }
    }

    pub const fn row(self) -> bool {
        self.row
    }

    pub const fn col(self) -> bool {
        self.col
    }

    pub const fn visited(self) -> bool {
        self.row || self.col
    }

    pub fn visit_row(&mut self) -> bool {
        mem::replace(&mut self.row, true)
    }

    pub fn visit_col(&mut self) -> bool {
        mem::replace(&mut self.col, true)
    }
}

fn get_entry_point_energy(grid: &Grid<u8>, coordinate: UCoordinate<2>, direction: Direction) -> usize {
    let mut progress = Grid::full(grid.rows(), grid.cols(), Visited::new());

    // Could have just been a Vec
    let mut operations = VecDeque::new();
    operations.push_back((coordinate, direction));

    while let Some((coordinate, direction)) = operations.pop_front() {
        let visited = &mut progress[coordinate];
        let c = grid[coordinate] as char;
        let is_mirror = c == '/' || c == '\\';
        match direction {
            Direction::North | Direction::South => if visited.visit_col() && !is_mirror {
                continue
            },
            Direction::East | Direction::West => if visited.visit_row() && !is_mirror {
                continue
            },
        }

        let mut add_operation = |dir: Direction| {
            let Some(mut next) = coordinate.checked_add(&dir) else {
                return;
            };
            grid.bound_coordinate(&mut next);
            if next != coordinate {
                // println!("{coordinate:?} -> ({dir:?}) {next:?}");
                operations.push_back((next, dir));
            }
        };

        let new_direction = match c {
            '.' => direction,
            '/' => direction.transpose_secondary(),
            '\\' => direction.transpose(),
            '-' => {
                match direction {
                    Direction::East | Direction::West => direction,
                    Direction::North | Direction::South => {
                        add_operation(Direction::East);
                        Direction::West
                    }
                }
            },
            '|' => {
                match direction {
                    Direction::North | Direction::South => direction,
                    Direction::East | Direction::West => {
                        add_operation(Direction::North);
                        Direction::South
                    }
                }
            },
            _ => panic!("Unknown char {c}"),
        };
        add_operation(new_direction);
    }
    // print_grid(&progress);
    progress.iter_rows()
        .flat_map(|row| row.iter().copied())
        .filter(|v| v.visited())
        .count()
}

fn part_1(grid: &Grid<u8>) -> usize {
    get_entry_point_energy(grid, UCoordinate::origin(), Direction::East)
}

fn part_2(grid: &Grid<u8>) -> usize {
    possible_entry_points(grid.rows(), grid.cols())
        .map(|(coordinate, direction)| {
            get_entry_point_energy(grid, coordinate, direction)
        })
        .max().unwrap()
}

fn possible_entry_points(rows: usize, cols: usize) -> impl Iterator<Item=(UCoordinate<2>, Direction)> {
    (0..rows).into_iter()
        .map(move |row| ((row, 0).into(), Direction::East))
        .chain((0..rows).into_iter()
            .map(move |row| ((row, cols - 1).into(), Direction::West))
        )
        .chain((0..cols).into_iter()
            .map(move |col| ((0, col).into(), Direction::South))
        )
        .chain((0..cols).into_iter()
            .map(move |col| ((rows - 1, col).into(), Direction::North))
        )
}

#[cfg(test)]
mod test {
    use super::*;

}
