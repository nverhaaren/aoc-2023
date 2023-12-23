use std::{io, iter, mem, str};
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::hash::Hash;
use std::str::FromStr;
use anyhow::anyhow;
use itertools::Itertools;
use aoc_2023::coordinate::{UCoordinate};
use aoc_2023::util::{FromStrParser, get_lines_from_stdin, Parser};

type Point = UCoordinate<3>;
type Point2 = UCoordinate<2>;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Block {
    start: Point,
    end: Point,
    axis: usize,
}

fn main() -> Result<(), Box<dyn Error>> {
    let lines = get_lines_from_stdin()?;
    let blocks = FromStrParser::<Block>::new()
        .parse_lines_to_vec(lines.iter().map(|s| s.as_str()))?;
    println!("Part 1: {}", part_1(&blocks));
    Ok(())
}

fn part_1(blocks: &[Block]) -> usize {
    let mut sections: HashMap<Point2, Vec<_>> = HashMap::new();
    for (idx, block) in blocks.iter().enumerate() {
        for shadow in block.shadow() {
            sections.entry(shadow).or_default().push((block.bottom(), idx));
        }
    }

    for val in sections.values_mut() {
        val.sort();
    }

    let sections = sections;

    let mut above: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut below: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut zero_below: HashSet<_> = (0..blocks.len()).into_iter().collect();
    let mut below_count: HashMap<usize, usize> = HashMap::new();

    for seq in sections.values() {
        for ((_, low), (_, high)) in seq.iter().copied().tuple_windows() {
            above.entry(low).or_default().insert(high);
            if below.entry(high).or_default().insert(low) {
                *below_count.entry(high).or_default() += 1;
            }
            zero_below.remove(&high);
        }
    }

    let above = above;
    let below = below;
    let mut can_destroy = HashSet::new();
    let mut moved_blocks: HashMap<usize, Block> = HashMap::new();

    let empty: HashSet<usize> = HashSet::new();

    let mut moved_count = 0usize;

    while let Some(idx) = zero_below.iter().copied().next() {
        let mut block = blocks[idx];
        zero_below.remove(&idx);
        assert_eq!(below_count.get(&idx).copied().unwrap_or_default(), 0);

        let mut max_top = 0usize;
        let mut tied = false;
        let mut support = usize::MAX;
        for below_idx in below.get(&idx).unwrap_or(&empty).iter().copied() {
            let challenger = moved_blocks.get(&below_idx).expect("logic error").top();
            if support == usize::MAX {
                support = below_idx;
                max_top = challenger;
                continue
            }
            match max_top.cmp(&challenger) {
                Ordering::Equal => tied = true,
                Ordering::Greater => (),
                Ordering::Less => {
                    support = below_idx;
                    max_top = challenger;
                    tied = false;
                }
            }
        }

        block.lower(block.bottom().checked_sub(max_top + 1).expect("collision"));
        assert!(moved_blocks.insert(idx, block).is_none());
        if !tied && support != usize::MAX {
            can_destroy.remove(&support);
        }
        can_destroy.insert(idx);

        for above_idx in above.get(&idx).unwrap_or(&empty) {
            let count = below_count.get_mut(&above_idx).unwrap();
            *count = count.checked_sub(1).expect("logic error");
            if *count == 0 {
                assert!(zero_below.insert(*above_idx));
            }
        }

        moved_count += 1;
    }

    assert_eq!(moved_count, blocks.len());

    can_destroy.len()
}

impl Block {
    pub fn shadow(&self) -> Box<dyn Iterator<Item=Point2> + '_> {
        if self.axis == 2 {
            Box::new(iter::once((self.start.as_ref()[0], self.start.as_ref()[1]).into()))
        } else {
            let mut shadow: Point2 = [self.start.as_ref()[0], self.start.as_ref()[1]].into();
            let increment: Point2 = if self.axis == 0 { [1, 0].into() } else { [0, 1].into() };
            Box::new((0..=(self.end.as_ref()[self.axis] - self.start.as_ref()[self.axis]))
                .into_iter()
                .map(move |_| {
                    let result = shadow;
                    shadow = shadow + increment;
                    result
                })
            )
        }
    }

    pub fn lower(&mut self, distance: usize) -> &mut Self {
        let start_z = &mut self.start.as_mut()[2];
        *start_z = start_z.checked_sub(distance).expect("logic error");
        let end_z = &mut self.end.as_mut()[2];
        *end_z = end_z.checked_sub(distance).expect("logic error");
        self
    }

    pub fn bottom(&self) -> usize {
        self.start.as_ref()[2]
    }

    pub fn top(&self) -> usize {
        self.end.as_ref()[2]
    }

    pub fn try_new(a: Point, b: Point) -> Result<Self, anyhow::Error> {
        if a == b {
            return Ok(Self { start: a, end: b, axis: 2 })
        }
        if a.as_ref()[0] != b.as_ref()[0] {
            if a.as_ref()[1..] == b.as_ref()[1..] {
                let start = a.min(b);
                let end = a.max(b);
                Ok(Self { start, end, axis: 0 })
            } else {
                Err(anyhow!("Not 1-D {a:?} {b:?}"))
            }
        } else if a.as_ref()[1] != b.as_ref()[1] {
            if a.as_ref()[0] == b.as_ref()[0] && a.as_ref()[2] == b.as_ref()[2] {
                let start = a.min(b);
                let end = a.max(b);
                Ok(Self { start, end, axis: 1 })
            } else {
                Err(anyhow!("Not 1-D {a:?} {b:?}"))
            }
        } else if a.as_ref()[2] != b.as_ref()[2] {
            if a.as_ref()[..2] == b.as_ref()[..2] {
                let start = a.min(b);
                let end = a.max(b);
                Ok(Self { start, end, axis: 2 })
            } else {
                Err(anyhow!("Not 1-D {a:?} {b:?}"))
            }
        } else {
            Err(anyhow!("Not 1-D {a:?} {b:?}"))
        }
    }
}

impl FromStr for Block {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s.split_once('~').ok_or(anyhow!("No '~'"))?;
        let (a1, a_rest) = a.split_once(',').ok_or(anyhow!("No ','"))?;
        let (a2, a3) = a_rest.split_once(',').ok_or(anyhow!("Expected ','"))?;
        let a = Point::new([a1.parse()?, a2.parse()?, a3.parse()?]);

        let (b1, b_rest) = b.split_once(',').ok_or(anyhow!("No ','"))?;
        let (b2, b3) = b_rest.split_once(',').ok_or(anyhow!("Expected ','"))?;
        let b = Point::new([b1.parse()?, b2.parse()?, b3.parse()?]);
        Ok(Block::try_new(a, b)?)
    }
}

#[cfg(test)]
mod test {
    use super::*;

}
