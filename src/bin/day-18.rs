use std::{io, str};
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use itertools::Itertools;
use regex::Regex;
use aoc_2023::coordinate::{Direction, ICoordinate, twice_shoelace};

fn main() {
    let stdin = io::stdin();
    let reader = BufReader::with_capacity(256, stdin.lock());
    let plans: Vec<_> = reader.lines()
        .map(|r| r.map_err(|_| ()).and_then(|line| -> Result<Plan, _> { line.parse() } ))
        .try_collect().expect("Unicode issue");
    println!("First part: {}", part_1(&plans));
    println!("Second part: {}", part_2(&plans));
}

type Point = ICoordinate<2>;

fn part_1(plans: &[Plan]) -> usize {
    let boundary: usize = plans.iter().map(|plan| plan.length).sum();
    let mut current = Point::origin();
    let twice_area = twice_shoelace(plans.iter()
        .map(move |plan| {
            let mut trench: Point = plan.direction.into();
            for x in trench.as_mut().iter_mut() {
                *x *= plan.length as isize;
            }
            current = current + trench;
            current
        }));
    let diff = twice_area.checked_sub(boundary).expect("Formula issue");
    assert_eq!(diff % 2, 0, "math issue");
    (diff / 2 + 1) + boundary
}

fn part_2(plans: &[Plan]) -> usize {
    todo!()
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Plan {
    direction: Direction,
    length: usize,
    rgb: (u8, u8, u8),
}

impl FromStr for Plan {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^([RLUD]) (\d+) \(#([0-9a-f]{2})([0-9a-f]{2})([0-9a-f]{2})\)$").unwrap();
        let (_, [dir, len, r, g, b]) = re.captures(s).ok_or(())?.extract();
        let direction = match dir {
            "R" => Direction::East,
            "L" => Direction::West,
            "U" => Direction::North,
            "D" => Direction::South,
            _ => unreachable!(),
        };
        let length: usize = len.parse().map_err(|_| ())?;
        let red = u8::from_str_radix(r, 16).map_err(|_| ())?;
        let green = u8::from_str_radix(g, 16).map_err(|_| ())?;
        let blue = u8::from_str_radix(b, 16).map_err(|_| ())?;
        Ok(Self { direction, length, rgb: (red, green, blue) })
    }
}

#[cfg(test)]
mod test {
    use super::*;

}
