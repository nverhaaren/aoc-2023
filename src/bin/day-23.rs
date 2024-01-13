use std::{iter, str};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::error::Error;
use std::hash::Hash;
use std::str::FromStr;
use anyhow::{anyhow, bail};
use itertools::Itertools;
use topological_sort::TopologicalSort;
use aoc_2023::coordinate::Direction;
use aoc_2023::coordinate::grid::{get_byte_grid_from_stdin, Point};
use aoc_2023::util::{CheckedSub, FromStrParser, get_lines_from_stdin, Parser};

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

    let mut graph = HashMap::new();
    let mut sort: TopologicalSort<Point> = TopologicalSort::new();

    {
        // Build graph
        let mut next_routes = vec![(source + Direction::South, Direction::South)];
        let mut explored = HashSet::new();
        while let Some((next_source, next_direction)) = next_routes.pop() {
            assert!(explored.insert((next_source, next_direction)));
            let (dest, dest_direction, length) = grid.follow_path(&next_source, next_direction, is_path);
            // println!("{next_source:?} -> {dest:?} {dest_direction:?} {length}");
            let graph_source = next_source.checked_sub(&next_direction).unwrap();
            graph.entry(dest).or_insert(vec![]).push((graph_source, next_direction, length + 1, dest_direction));
            sort.add_dependency(graph_source, dest);
            for (dir, neighbor) in grid.neighbors(&dest) {
                if dir.opposite() == dest_direction { continue; }
                if convert_to_direction(grid[neighbor])? == Some(dir) && !explored.contains(&(neighbor, dir)) {
                    next_routes.push((neighbor, dir));
                }
            }
        }
    }

    let graph = graph;
    let mut max_distances = HashMap::new();
    max_distances.insert(source, 0usize);
    while let Some(point) = sort.pop() {
        let Some(v) = graph.get(&point) else {
            continue;  // Inaccessible I think - is that allowed? Also would cover start.
        };
        let mut max_distance = 0usize;
        for (source, _, length, _) in v {
            let Some(source_dist) = max_distances.get(source) else {
                continue;
            };
            max_distance = max_distance.max(*length + *source_dist);
        }
        assert!(max_distances.insert(point, max_distance).is_none());
    }

    println!(
        "Part 1: {}",
        max_distances
            .get(&(grid.rows() - 1, grid.cols() - 2).into())
            .ok_or(anyhow!("No path to endpoint"))?
    );
    Ok(())
}

fn parts() {
   todo!()
}

#[cfg(test)]
mod test {
    use super::*;

}
