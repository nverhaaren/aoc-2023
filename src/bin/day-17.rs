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
const MAX_STREAK_B: usize = 10;
const MIN_STREAK_B: usize = 4;

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

    pub fn origin_state_b() -> Self {
        Self([
            // South and east allowed turns immediately after the start; this forbids them
            ((Direction::North, 4), 0),
            ((Direction::West, 4), 0),
        ].iter().copied().collect())
    }

    pub fn min_heat_loss(&self) -> usize {
        self.0.values().copied().min().unwrap_or(usize::MAX)
    }

    pub fn min_heat_loss_b(&self) -> usize {
        self.0.iter()
            .filter(|((_, streak), _)| {
                assert!(*streak <= MAX_STREAK_B);
                *streak >= MIN_STREAK_B
            })
            .map(|(_, loss)| *loss)
            .min()
            .unwrap_or(usize::MAX)
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

    pub fn relevant_loss_jump_b(&self, towards: Direction, jump_loss: usize) -> usize {
        let mut min_loss_from_other_directions = usize::MAX;
        for (&(streak_direction, _streak), &loss) in self.0.iter() {
            let loss = loss + jump_loss;
            if towards == streak_direction.opposite() || towards == streak_direction { // Problem statement?
                continue;
            }
            if towards != streak_direction {
                min_loss_from_other_directions = min_loss_from_other_directions.min(loss);
            }
        }
        min_loss_from_other_directions
    }

    pub fn relevant_losses_adjacent_b(&self, towards: Direction, neighbor_loss: usize) -> Vec<(usize, usize)> {
        let mut losses_from_same_direction = vec![];
        for (&(streak_direction, streak), &loss) in self.0.iter() {
            if streak_direction != towards {
                continue;
            }
            if loss == usize::MAX {
                continue;
            }
            let loss = loss + neighbor_loss;
            if streak == MAX_STREAK_B {
                continue;
            }
            losses_from_same_direction.push((streak + 1, loss));
        }
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
            assert!(streak <= MAX_STREAK_B);
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
    let mut state = Grid::full(grid.rows(), grid.cols(), PathState::new());
    state[UCoordinate::origin()] = PathState::origin_state_b();

    let mut causing_update: HashSet<_> = [
        UCoordinate::<2>::origin()
    ].iter().copied().collect();

    while let Some(dirty) = causing_update.iter().next().copied() {
        causing_update.remove(&dirty);
        for (dir, neighbor) in grid.neighbors(&dirty) {
            let neighbor_loss = grid[neighbor];
            let relevant_losses = state[dirty].relevant_losses_adjacent_b(dir, neighbor_loss);
            if state[neighbor].update_state(dir, &relevant_losses) {
                causing_update.insert(neighbor);
            }
        }

        for (dir, neighbor) in jump_neighbors(grid, &dirty) {
            let mut next = dirty + dir;
            let mut path_loss = 0usize;
            loop {
                path_loss += grid[next];
                if next == neighbor {
                    break;
                }
                next = next + dir;
            }
            let min_loss = [(4, state[dirty].relevant_loss_jump_b(dir, path_loss))];
            if min_loss[0].1 == usize::MAX {
                 continue
            }
            if state[neighbor].update_state(dir, &min_loss) {
                causing_update.insert(neighbor);
            }
        }
    }

    state[*state.bound_coordinate(&mut (usize::MAX, usize::MAX).into())].min_heat_loss_b()
}

fn jump_neighbors<'a>(grid: &'a Grid<usize>, coordinate: &UCoordinate<2>) -> impl Iterator<Item=(Direction, UCoordinate<2>)> + 'a {
    let coordinate = *coordinate;
    Direction::ALL.iter().copied()
        .filter_map(move |d| {
            match d {
                Direction::North => {
                    grid.checked_sub(&coordinate, &UCoordinate::from((MIN_STREAK_B, 0)))
                },
                Direction::East => {
                    grid.checked_add(&coordinate, &UCoordinate::from((0, MIN_STREAK_B)))
                },
                Direction::South => {
                    grid.checked_add(&coordinate, &UCoordinate::from((MIN_STREAK_B, 0)))
                },
                Direction::West => {
                    grid.checked_sub(&coordinate, &UCoordinate::from((0, MIN_STREAK_B)))
                },
            }.map(|coord| (d, coord))
        })
}

#[cfg(test)]
mod test {
    use super::*;

}
