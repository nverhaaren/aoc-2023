use std::{io, mem};
use std::io::{BufRead, BufReader};
use itertools::Itertools;
use regex::Regex;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Tile {
    Pipe(Pipe),
    Ground,
    Start,
}

impl TryFrom<char> for Tile {
    type Error = char;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '|' => Tile::Pipe(Pipe::NS),
            '-' => Tile::Pipe(Pipe::EW),
            'L' => Tile::Pipe(Pipe::NE),
            'J' => Tile::Pipe(Pipe::NW),
            '7' => Tile::Pipe(Pipe::SW),
            'F' => Tile::Pipe(Pipe::SE),
            '.' => Tile::Ground,
            'S' => Tile::Start,
            _ => return Err(value),
        })
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Pipe {
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
}

impl From<Pipe> for (Direction, Direction) {
    fn from(value: Pipe) -> Self {
        match value {
            Pipe::NS => (Direction::North, Direction::South),
            Pipe::EW => (Direction::East, Direction::West),
            Pipe::NE => (Direction::North, Direction::East),
            Pipe::NW => (Direction::North, Direction::West),
            Pipe::SW => (Direction::South, Direction::West),
            Pipe::SE => (Direction::South, Direction::East),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

type Coordinate = (usize, usize);
impl TryFrom<(Coordinate, Coordinate)> for Direction {
    type Error = (Coordinate, Coordinate);
    fn try_from(value: (Coordinate, Coordinate)) -> Result<Self, Self::Error> {
        let (source, dest) = value;
        if source.0 == dest.0 {
            if source.1 + 1 == dest.1 {
                Ok(Direction::East)
            } else if dest.1 + 1 == source.1 {
                Ok(Direction::West)
            } else {
                Err(value)
            }
        } else if source.1 == dest.1 {
            if source.0 + 1 == dest.0 {
                Ok(Direction::South)
            } else if dest.0 + 1 == source.0 {
                Ok(Direction::North)
            } else {
                Err(value)
            }
        } else {
            Err(value)
        }
    }
}

fn adjacent_coordinates(c: Coordinate, rows: usize, cols: usize) -> impl Iterator<Item=Coordinate> {
    (0usize..9).into_iter()
        .map(|x| (x / 3, x % 3))
        .filter(move |&(row, col)| {
            if (row, col) == (1, 1) {
                return false;
            } else if c.0 == 0 && row == 0 {
                return false;
            } else if c.1 == 0 && col == 0 {
                return false;
            } else if c.0 == rows - 1 && row == 2 {
                return false;
            } else if c.1 == cols - 1 && col == 2 {
                return false;
            }
            true
        })
        .map(move |(row, col)| (c.0 + row - 1, c.1 + col - 1))
}

//////

fn process_lines(lines: impl Iterator<Item=String>) -> i64 {
    let map: Vec<_> = lines
        .map(|line| -> Vec<Tile> {
            line.chars()
                .map(|c| Tile::try_from(c))
                .try_collect().expect("Invalid input character")
        })
        .collect();
    let rows = map.len();
    let cols = map[0].len();
    assert!(map.iter().all(|r| r.len() == cols));

    let start = find_start(map.as_slice());

    // todo: infer pipe at start
    todo!()
}

fn find_start(map: &[Vec<Tile>]) -> Coordinate {
    map.iter()
        .enumerate()
        .flat_map(|(row_idx, row)| {
            row.iter()
                .copied()
                .enumerate()
                .map(move |(col_idx, tile)| (row_idx, col_idx, tile))
        })
        .find_map(|(r, c, tile)| if tile == Tile::Start {
            Some((r, c))
        } else {
            None
        })
        .expect("Could not find any start point")
}

fn main() {
    let stdin = io::stdin();
    let reader = BufReader::with_capacity(256, stdin.lock());
    println!("{}", process_lines(reader.lines()
        .map(|s| s.expect("unicode issue"))))
}
