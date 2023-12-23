use std::mem;
use std::collections::HashMap;
use std::error::Error;
use anyhow::anyhow;
use aoc_2023::coordinate::UCoordinate;
use aoc_2023::coordinate::grid::{get_byte_grid_from_stdin, Grid};

type Point = UCoordinate<2>;

fn main() -> Result<(), Box<dyn Error>> {
    let grid = get_byte_grid_from_stdin()?;
    let starting_position = grid.iter_idxs()
        .filter(|idx| grid[*idx] as char == 'S')
        .next().ok_or(anyhow!("Could not find S"))?;

    println!("Part 1: {}", part_1(&grid, starting_position));
    Ok(())
}

// const STEPS: usize = 6;
const STEPS: usize = 64;

fn part_1(grid: &Grid<u8>, starting_position: Point) -> usize {
    let mut distances = HashMap::new();
    distances.insert(starting_position, 0usize);

    let mut next_distance = 1usize;
    let mut frontier = vec![starting_position];
    let mut next_frontier = vec![];
    while next_distance <= STEPS {
        for point in frontier.iter().copied() {
            for (_, neighbor) in grid.neighbors(&point) {
                if grid[neighbor] as char == '#' { continue; }
                let current = distances.entry(neighbor).or_insert(usize::MAX);
                if next_distance >= *current { continue; }
                *current = next_distance;
                next_frontier.push(neighbor);
            }
        }
        mem::swap(&mut frontier, &mut next_frontier);
        next_frontier.clear();
        next_distance += 1;
    }
    grid.iter_idxs()
        .filter(|idx| {
            distances.get(idx).copied()
                .map(|dist| dist <= STEPS && dist % 2 == 0)
                .unwrap_or(false)
        })
        .count()
}

#[cfg(test)]
mod test {
    use super::*;

}
