use std::{io};
use std::io::{BufRead, BufReader};
use itertools::Itertools;


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

#[allow(unused)]
impl Direction {
    pub fn opposite(self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
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

#[allow(dead_code)]
fn are_adjacent(c1: Coordinate, c2: Coordinate) -> bool {
    if c1.0 == c2.0 {
        c1.1.abs_diff(c2.1) == 1
    } else if c1.1 == c2.1 {
        c1.0.abs_diff(c2.0) == 1
    } else {
        false
    }
}

fn go_direction(c: Coordinate, d: Direction) -> Option<Coordinate> {
    match d {
        Direction::North => if c.0 > 0 {
            Some((c.0 - 1, c.1))
        } else {
            None
        },
        Direction::South => Some((c.0 + 1, c.1)),
        Direction::West => if c.1 > 0 {
            Some((c.0, c.1 - 1))
        } else {
            None
        },
        Direction::East => Some((c.0, c.1 + 1)),
    }
}

//////

fn process_lines(lines: impl Iterator<Item=String>) -> u64 {
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

    let adjacent: Vec<_> = adjacent_coordinates(start, rows, cols)
        .filter(|c| {
            goes_to(map.as_slice(), *c, start)
        })
        .collect();
    assert_eq!(adjacent.len(), 2, "puzzle constraint violated");

    let mut last = start;
    let mut current = adjacent[0];
    let mut count = 2u64;
    while current != adjacent[1] {
        // println!("Debug: {current:?}");
        let Tile::Pipe(pipe) = map[current.0][current.1] else {
            panic!("Not a tile at {current:?}");
        };
        let (d1, d2): (Direction, Direction) = pipe.into();
        let back: Direction = (current, last).try_into().expect("Not adjacent");
        let forward = if back == d1 {
            assert_ne!(back, d2);
            d2
        } else {
            assert_eq!(back, d2);
            d1
        };
        // println!("Debug: {forward:?}");
        last = current;
        current = go_direction(current, forward).expect("Out of bounds");
        count += 1
    }
    count / 2 + count % 2
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

// fn infer_start(&map: &[Vec<Tile>], start_coordinate: Coordinate) -> Result<Pipe, ()> {
//     // iter adjacent, check goes to, get two directions, get pipe
// }

fn goes_to(map: &[Vec<Tile>], source: Coordinate, dest: Coordinate) -> bool {
    match map[source.0][source.1] {
        Tile::Pipe(pipe) => {
            let dirs: (Direction, Direction) = pipe.into();
            go_direction(source, dirs.0) == Some(dest) ||
                go_direction(source, dirs.1) == Some(dest)
        },
        _ => false,
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = BufReader::with_capacity(256, stdin.lock());
    println!("{}", process_lines(reader.lines()
        .map(|s| s.expect("unicode issue"))))
}
