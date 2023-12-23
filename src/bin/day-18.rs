use std::{io, str};
use std::io::{BufRead, BufReader};
use itertools::Itertools;
use regex::Regex;
use aoc_2023::coordinate::{Direction, ICoordinate, twice_shoelace};

fn main() {
    let stdin = io::stdin();
    let reader = BufReader::with_capacity(256, stdin.lock());
    let lines: Vec<_> = reader.lines()
        .try_collect().expect("Unicode issue");
    let plans_1: Vec<Plan> = lines.iter().map(|line| Plan::parse_1(line)).collect();
    let plans_2: Vec<Plan> = lines.iter().map(|line| Plan::parse_2(line)).collect();
    println!("First part: {}", compute_covered(&plans_1));
    println!("Second part: {}", compute_covered(&plans_2));
}

type Point = ICoordinate<2>;

fn compute_covered(plans: &[Plan]) -> usize {
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

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Plan {
    direction: Direction,
    length: usize,
}

impl Plan {
    pub fn parse_1(s: &str) -> Self {
        let re = Regex::new(r"^([RLUD]) (\d+) \(#[0-9a-f]{6}\)$").unwrap();
        let (_, [dir, len]) = re.captures(s).expect("input issue").extract();
        let direction = match dir {
            "R" => Direction::East,
            "L" => Direction::West,
            "U" => Direction::North,
            "D" => Direction::South,
            _ => unreachable!(),
        };
        let length: usize = len.parse().unwrap();

        Self { direction, length }
    }

    pub fn parse_2(s: &str) -> Self {
        let re = Regex::new(r"^[RLUD] \d+ \(#([0-9a-f]{5})([0-9a-f])\)$").unwrap();
        let (_, [len, dir]) = re.captures(s).expect("input issue").extract();
        let direction = match dir {
            "0" => Direction::East,
            "2" => Direction::West,
            "3" => Direction::North,
            "1" => Direction::South,
            _ => unreachable!(),
        };
        let length: usize = usize::from_str_radix(len, 16).unwrap();

        Self { direction, length }
    }
}
