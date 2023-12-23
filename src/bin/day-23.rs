use std::{iter, str};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::error::Error;
use std::hash::Hash;
use std::str::FromStr;
use anyhow::{anyhow, bail};
use itertools::Itertools;
use aoc_2023::coordinate::Direction;
use aoc_2023::coordinate::grid::{get_byte_grid_from_stdin, Point};
use aoc_2023::util::{FromStrParser, get_lines_from_stdin, Parser};

fn convert_to_direction(b: u8) -> Result<Option<Direction>, anyhow::Error> {
    match b as char {
        '#' => Ok(None),
        '.' => Err(anyhow!("DAG invariant violated")),
        '>' => Ok(Some(Direction::East)),
        'v' => Ok(Some(Direction::South)),
        '^' => Ok(Some(Direction::North)),
        '<' => Ok(Some(Direction::West)),
        _ => Err(anyhow!("Not recognized: {b:?}")),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let grid = get_byte_grid_from_stdin()?;
    let is_path = |_: &Point, b: &u8| *b as char != '#';
    let source: Point = (0, 1).into();
    let mut next_routes = vec![(source, Direction::South)];
    let mut graph = HashMap::new();
    let mut explored = HashSet::new();

    // Build graph
    while let Some((next_source, next_direction)) = next_routes.pop() {
        assert!(explored.insert((next_source, next_direction)));
        let (dest, dest_direction, length) = grid.follow_path(&next_source, next_direction, is_path);
        graph.entry(next_source).or_insert(vec![]).push((next_direction, length, dest, dest_direction));
        for (dir, neighbor) in grid.neighbors(&dest) {
            if dir.opposite() == dest_direction { continue; }
            if convert_to_direction(grid[neighbor])? == Some(dir) && !explored.contains(&(neighbor, dir)) {
                next_routes.push((neighbor, dir));
            }
        }
    }
    todo!()
}

fn parts() {
   todo!()
}

#[cfg(test)]
mod test {
    use super::*;

}
