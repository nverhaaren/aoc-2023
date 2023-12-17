use std::{io, iter, mem};
use std::collections::VecDeque;
use std::fmt::{Display, Formatter, Write};
use std::io::{BufRead, BufReader};
use itertools::Itertools;
use aoc_2023::coordinate::{Direction, Grid, UCoordinate};
use aoc_2023::util::CheckedAdd;

fn main() {
    let stdin = io::stdin();
    let reader = BufReader::with_capacity(256, stdin.lock());
    let lines: Vec<_> = reader.lines()
        .map(|x| x.map(|s| s.into_bytes()))
        .try_collect().expect("Unicode issue");
    println!("First part: {}", part_1(&lines));
    println!("Second part: {}", part_2(&lines));
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

fn get_entry_point_energy(lines: &Vec<Vec<u8>>, coordinate: UCoordinate<2>, direction: Direction) -> usize {
    let mut progress: Vec<_> = iter::repeat_with(|| vec![Visited::new(); lines[0].len()])
        .take(lines.len())
        .collect();

    // Could have just been a Vec
    let mut operations = VecDeque::new();
    operations.push_back((coordinate, direction));

    while let Some((coordinate, direction)) = operations.pop_front() {
        let (row, col): (usize, usize) = coordinate.into();
        let visited = &mut progress[row][col];
        let c = lines[row][col] as char;
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
            next.bound_axis(0, 0..lines.len()).expect("logic error");
            next.bound_axis(1, 0..lines[0].len()).expect("logic error");
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
    progress.iter()
        .flat_map(|row| row.iter().copied())
        .filter(|v| (*v).visited())
        .count()
}

fn part_1(lines: &Vec<Vec<u8>>) -> usize {
    get_entry_point_energy(lines, UCoordinate::origin(), Direction::East)
}

fn part_2(lines: &Grid<u8>) -> usize {
    possible_entry_points(lines.len(), lines[0].len())
        .map(|(coordinate, direction)| {
            get_entry_point_energy(lines, coordinate, direction)
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

#[allow(unused)]
fn print_grid<T: Display>(grid: &Grid<T>) {
    for row in grid.iter() {
        for cell in row.iter() {
            print!("{cell}");
        }
        println!();
    }
}

#[cfg(test)]
mod test {
    use super::*;

}
