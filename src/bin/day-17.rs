use std::{io, };
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Write};
use std::io::{BufRead, BufReader};
use itertools::Itertools;
use aoc_2023::coordinate::{Direction, Grid, UCoordinate};
use aoc_2023::util::CheckedAdd;

fn main() {
    let stdin = io::stdin();
    let reader = BufReader::with_capacity(256, stdin.lock());
    let lines: Vec<_> = reader.lines()
        .map(|x| x.map(|s| -> Vec<usize> {
            s.bytes()
                .map(|b| (b as char).to_digit(10).expect("Invalid input") as usize)
                .collect()
        }))
        .try_collect().expect("Unicode issue");
    let grid = Grid::try_from_vec_of_vecs(lines).expect("Irregular input");
    println!("First part: {}", part_1(&grid));
    println!("Second part: {}", part_2(&grid));
}

const MAX_STREAK: usize = 3; // Problem statement

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct PathState(HashMap<(Direction, usize), usize>);

impl PathState {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn origin_state() -> Self {
        Self([
            ((Direction::South, 1), 0),
            ((Direction::East, 1), 0),
        ].iter().copied().collect())
    }

    pub fn min_heat_loss(&self) -> usize {
        self.0.values().copied().min().unwrap_or(usize::MAX)
    }

    pub fn relevant_losses(&self, towards: Direction, neighbor_loss: usize) -> Vec<(usize, usize)> {
        let mut min_loss_from_other_directions = usize::MAX;
        let mut losses_from_same_direction = vec![];
        for (&(streak_direction, streak), &loss) in self.0.iter() {
            let loss = loss + neighbor_loss;
            if towards == streak_direction.opposite() { // Problem statement
                continue;
            }
            if towards != streak_direction {
                min_loss_from_other_directions = min_loss_from_other_directions.min(loss);
                continue;
            }
            if streak == MAX_STREAK {
                continue;
            }
            losses_from_same_direction.push((streak + 1, loss));
        }
        losses_from_same_direction.push((1, min_loss_from_other_directions));
        losses_from_same_direction.sort();
        let mut min_loss = usize::MAX;
        losses_from_same_direction.retain(|&(_, loss)| {
            // Ignore strictly worse paths (same direction, equal or more heat loss, longer streak)
            let result = loss < min_loss;
            min_loss = min_loss.min(loss);
            result
        });
        losses_from_same_direction
    }

    /// Returns whether this improved our path at all
    /// Going direction from neighbor takes you to self
    /// heat_loss is the heat_loss of self
    pub fn update_state(&mut self, direction: Direction, relevant_losses: &[(usize, usize)]) -> bool {
        let mut updated = false;
        for (streak, loss) in relevant_losses.iter().copied() {
            let best_loss = self.0.entry((direction, streak))
                .or_insert(usize::MAX);
            if loss >= *best_loss {
                continue;
            }
            *best_loss = loss;
            updated = true;
        }
        updated
    }
}


fn part_1(grid: &Grid<usize>) -> usize {
    let mut state = Grid::full(grid.rows(), grid.cols(), PathState::new());
    state[UCoordinate::origin()] = PathState::origin_state();

    let mut causing_update: HashSet<_> = [
        UCoordinate::<2>::origin()
    ].iter().copied().collect();

    while let Some(dirty) = causing_update.iter().next().copied() {
        causing_update.remove(&dirty);
        for (dir, neighbor) in grid.neighbors(&dirty) {
            let neighbor_loss = grid[neighbor];
            let relevant_losses = state[dirty].relevant_losses(dir, neighbor_loss);
            if state[neighbor].update_state(dir, &relevant_losses) {
                causing_update.insert(neighbor);
            }
        }
    }

    state[*state.bound_coordinate(&mut (usize::MAX, usize::MAX).into())].min_heat_loss()
}

fn part_2(grid: &Grid<usize>) -> usize {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

}
