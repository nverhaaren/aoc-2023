use std::{io, iter};
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use anyhow::{anyhow, bail};
use itertools::Itertools;
use aoc_2023::number_theory::count_combinations;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum Mark {
    Works,
    Broken,
    Unknown,
}

impl TryFrom<char> for Mark {
    type Error = char;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '.' => Mark::Works,
            '#' => Mark::Broken,
            '?' => Mark::Unknown,
            _ => return Err(value),
        })
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
struct Line {
    marks: Vec<Mark>,
    seqs: Vec<usize>,
}

impl Line {
    #[allow(unused)]
    pub const fn empty() -> Self {
        Self { marks: vec![], seqs: vec![] }
    }

    pub fn count_combinations(&self) -> usize {
        let mut broken = 0usize;
        let mut unknown = 0usize;
        for &m in &self.marks {
            match m {
                Mark::Broken => broken += 1,
                Mark::Unknown => unknown += 1,
                _ => (),
            };
        };
        let total: usize = self.seqs.iter().sum();
        if broken > total {
            return 0;
        }
        let remaining = total - broken;
        count_combinations(unknown as u64, remaining as u64) as usize
    }

    pub fn reverse(&mut self) {
        self.marks.reverse();
        self.seqs.reverse();
    }

    pub fn valid_candidate(&self) -> Result<bool, ()> {
        let mut actual = vec![];
        let mut broken = 0usize;
        for m in self.marks.iter().copied() {
            match m {
                Mark::Works => if broken > 0 {
                    actual.push(broken);
                    broken = 0;
                },
                Mark::Broken => broken += 1,
                Mark::Unknown => return Err(()),
            }
        }
        if broken > 0 {
            actual.push(broken);
        }
        Ok(actual == self.seqs)
    }
}

impl FromStr for Line {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((marks, seqs)) = s.split_once(' ') else {
            bail!("Could not split on single space: {s:?}");
        };
        let marks: Vec<Mark> = marks.chars()
            .map(|c| -> Result<Mark, _> { c.try_into() })
            .try_collect()
            .map_err(|e| anyhow!("Unknown mark {e}"))?;
        let seqs: Vec<usize> = if seqs == "" { vec![] } else {
            seqs.split(',')
                .map(|s| -> Result<usize, _> { s.parse() })
                .try_collect()
                // Questions on most idiomatic here
                .map_err(|_| anyhow!("Could not parse int"))?
        };
        Ok(Self { marks, seqs })
    }
}

impl Line {
    pub fn reduce_left(&mut self) {
        let mut mark_idx = 0usize;
        let mut seq_idx = 0usize;
        let mut new_marks = vec![];
        let mut new_seqs = vec![];

        let mut broken_group = 0usize;
        while mark_idx < self.marks.len() {
            while mark_idx < self.marks.len() {
                let mark = self.marks[mark_idx];
                match mark {
                    Mark::Works => {
                        if broken_group > 0 {
                            assert_eq!(self.seqs[seq_idx], broken_group, "{mark_idx} {self:?}");
                            seq_idx += 1;
                            broken_group = 0;
                        }
                        mark_idx += 1;
                        continue;
                    },
                    Mark::Broken => broken_group += 1,
                    Mark::Unknown => if broken_group > 0 {
                        if broken_group < self.seqs[seq_idx] {
                            broken_group += 1;
                            mark_idx += 1;
                            continue;
                        }
                        assert_eq!(self.seqs[seq_idx], broken_group, "{mark_idx} {seq_idx} {broken_group} {self:?}");
                        broken_group = 0;
                        seq_idx += 1;
                    } else {
                        break
                    },
                }
                mark_idx += 1;
            }
            if mark_idx == self.marks.len() {
                if seq_idx != self.seqs.len() {
                    assert_eq!(seq_idx, self.seqs.len() - 1, "{self:?}");
                    assert_eq!(broken_group, self.seqs[seq_idx], "{self:?}");
                }
                self.marks = new_marks;
                self.seqs = new_seqs;
                return;
            }

            if seq_idx == self.seqs.len() {
                assert!(
                    (mark_idx..self.marks.len()).into_iter()
                        .all(|idx| self.marks[idx] == Mark::Works || self.marks[idx] == Mark::Unknown),
                    "{self:?}"
                );
                self.marks = new_marks;
                self.seqs = new_seqs;
                return;
            }

            assert_eq!(broken_group, 0);

            let mut any_known_broken = false;
            let unknown_broken_len = (mark_idx..self.marks.len()).into_iter()
                .take_while(|idx| {
                    self.marks[*idx] == Mark::Unknown || if self.marks[*idx] == Mark::Broken {
                        any_known_broken = true;
                        true
                    } else {
                        false
                    }
                })
                .count();

            if any_known_broken && unknown_broken_len == self.seqs[seq_idx] {
                mark_idx += unknown_broken_len;
                // seq_idx += 1;
                broken_group = unknown_broken_len;
                continue;
            }

            // Check for pattern ???### 3,
            let unknown_streak = (mark_idx..self.marks.len()).into_iter()
                .take_while(|idx| self.marks[*idx] == Mark::Unknown)
                .count();
            let broken_streak = ((mark_idx + unknown_streak)..self.marks.len()).into_iter()
                .take_while(|idx| self.marks[*idx] == Mark::Broken)
                .count();
            if broken_streak == self.seqs[seq_idx] {
                mark_idx += unknown_streak + broken_streak;
                // seq_idx += 1;
                broken_group = broken_streak;
            } else {
                break;
            }
        }
        new_marks.extend((mark_idx..self.marks.len()).into_iter().map(|idx| {
            self.marks[idx]
        }));
        new_seqs.extend((seq_idx..self.seqs.len()).into_iter().map(|idx| {
            self.seqs[idx]
        }));
        self.marks = new_marks;
        self.seqs = new_seqs;
        return;
    }

    pub fn reduce_right(&mut self) {
        // Would have been more efficient to implement left in terms of right, not vice versa
        self.reverse();
        self.reduce_left();
        self.reverse();
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct LineCombinationIter {
    line: Line,
    unknown_idxs: Vec<usize>,
    current_combination: Vec<usize>,
    combination_digit: Option<usize>,
}

impl LineCombinationIter {
    pub fn new(line: Line) -> Self {
        let mut broken = 0usize;
        let unknown_idxs: Vec<_> = line.marks.iter().copied()
            .inspect(|mark| if *mark == Mark::Broken { broken += 1; })
            .enumerate()
            .filter(|(_idx, mark)| *mark == Mark::Unknown)
            .map(|(idx, _)| idx)
            .collect();
        let required: usize = line.seqs.iter().sum();
        if broken > required {
            return Self {
                line: Line::from_str(" 1").unwrap(),
                unknown_idxs: vec![],
                current_combination: vec![],
                combination_digit: None,
            }
        }
        let remaining = required - broken;
        let current_combination: Vec<_> = (0..remaining).into_iter().collect();

        let mut unknown = 0usize;
        let mut new_line = line.clone();
        for mark in new_line.marks.iter_mut() {
            match *mark {
                Mark::Unknown => {
                    if unknown < remaining {
                        *mark = Mark::Broken;
                    } else {
                        *mark = Mark::Works;
                    }
                    unknown += 1;
                },
                _ => (),
            }
        };
        let combination_digit = if remaining == 0 {
            Some(0)
        } else {
            Some(current_combination.len() - 1)
        };
        let result = Self { line: new_line, unknown_idxs, current_combination, combination_digit };
        // println!("New: {result:?}");
        result
    }

    pub fn advance(&mut self) -> bool {
        assert!(self.unknown_idxs.len() >= self.current_combination.len(), "{self:?}");
        let Some(mut combination_digit) = self.combination_digit else {
            return false;
        };
        if self.current_combination.len() == 0 {
            self.combination_digit = None;
            return false;
        };
        loop {
            let max = self.unknown_idxs.len() - self.current_combination.len() + combination_digit;
            // println!("max ({max}/{combination_digit}) of self: {self:?}");
            let value = &mut self.current_combination[combination_digit];
            *value += 1;
            // println!("New value: {value}");
            if *value > max {
                if combination_digit == 0 {
                    // println!("No room to move at start: {:?}", self.line.marks);
                    self.combination_digit = None;
                    return false;
                }
                combination_digit -= 1;
            } else {
                let mut idx = combination_digit + 1;
                let mut new_value = *value + 1;
                while idx < self.current_combination.len() {
                    self.current_combination[idx] = new_value;
                    idx += 1;
                    new_value += 1;
                };
                self.combination_digit.replace(self.current_combination.len() - 1);
                self.apply_current_combination();
                return true;
            }
        }
    }

    fn apply_current_combination(&mut self) {
        assert!(self.current_combination.len() > 0, "{self:?}");
        if self.current_combination.len() == 1 {
            let value = self.current_combination[0];
            // println!("Setting broken in {:?}: {}, {}", self.line.marks, value, self.unknown_idxs[value]);
            self.line.marks[self.unknown_idxs[value]] = Mark::Broken;
        } else {
            for window in self.current_combination.windows(2) {
                assert_eq!(window.len(), 2);
                let start = window[0];
                let end = window[1];
                // println!("Setting broken in {:?}: {}, {}", self.line.marks, start, self.unknown_idxs[start]);
                // println!("Setting broken in {:?}: {}, {}", self.line.marks, end, self.unknown_idxs[end]);
                self.line.marks[self.unknown_idxs[start]] = Mark::Broken;
                self.line.marks[self.unknown_idxs[end]] = Mark::Broken;
                for idx in (start + 1)..(end) {
                    // println!("Setting fixed in {:?}: {}", self.line.marks, idx);
                    self.line.marks[self.unknown_idxs[idx]] = Mark::Works
                }
            }
        }
        for idx in 0..self.current_combination[0] {
            // println!("Setting fixed in {:?}: {} {}", self.line.marks, idx, self.unknown_idxs[idx]);
            self.line.marks[self.unknown_idxs[idx]] = Mark::Works;
        }
        for idx in (self.current_combination[self.current_combination.len() - 1] + 1)..self.unknown_idxs.len() {
            // println!("Setting fixed in {:?}: {} {}", self.line.marks, idx, self.unknown_idxs[idx]);
            self.line.marks[self.unknown_idxs[idx]] = Mark::Works;
        }
    }
}

fn process_lines(lines: impl Iterator<Item=String>) -> usize {
    lines
        .map(|line| -> String {
            let (first, second) = line.split_once(' ').expect("No space");
            let front: String = iter::repeat(first)
                .take(5)
                .intersperse("?")
                .collect();
            let back: String = iter::repeat(second)
                .take(5)
                .intersperse(",")
                .collect();
            format!("{front} {back}")
        })
        .map(|line| Line::from_str(line.as_str()).unwrap())
        // .map(|line| {
        //     let mut comb_iter = LineCombinationIter::new(line);
        //     let mut iter_count = 0usize;
        //     while comb_iter.combination_digit.is_some() {
        //         if comb_iter.line.valid_candidate().expect(&format!("Candidate generation issue: {:?}", comb_iter.line)) {
        //             iter_count += 1;
        //             // println!("Valid line: {:?}", comb_iter.line);
        //         } else {
        //             // println!("Invalid line: {:?}", comb_iter.line);
        //         }
        //         comb_iter.advance();
        //     }
        //     iter_count
        // })
        .map(|mut line| {
            line.reduce_left();
            line.reduce_right();
            line.count_combinations()
        } )
        .sum()
}


fn main() {
    let stdin = io::stdin();
    let reader = BufReader::with_capacity(256, stdin.lock());
    println!("{}", process_lines(reader.lines()
        .map(|s| s.expect("unicode issue"))))
}

#[cfg(test)]
mod test {
    use super::*;

    fn check_reduces_to(f: impl FnOnce(&mut Line), start: &str, end: &str) {
        let orig: Line = start.parse().unwrap();
        let expected: Line = end.parse().unwrap();
        let mut line = orig.clone();
        f(&mut line);
        assert_eq!(line, expected);
    }

    fn check_does_not_reduce(f: impl FnOnce(&mut Line), line: &str) {
        check_reduces_to(f, line, line);
    }

    #[test]
    fn test_reduce_left() {
        check_reduces_to(Line::reduce_left, "???.### 1,1,3", "???.### 1,1,3");
        check_does_not_reduce(Line::reduce_left, "??..??...?##. 1,1,3");
        check_does_not_reduce(Line::reduce_left, "?#?#?#?#?#?#?#? 1,3,1,6");
        check_does_not_reduce(Line::reduce_left, "????.######..#####. 1,6,5");
        check_does_not_reduce(Line::reduce_left, "?###???????? 3,2,1");
        // Indicates that there is only one possibility
        check_reduces_to(Line::reduce_left, "????.#...#... 4,1,1", " ");
    }

    #[test]
    fn test_count_combinations() {
        assert_eq!(Line::from_str("???.### 1,1,3").unwrap().count_combinations(), 3);
    }

    #[test]
    fn test_line_combination_iter_new() {
        LineCombinationIter::new("?? 1".parse().unwrap());
        LineCombinationIter::new("? 1".parse().unwrap());
        LineCombinationIter::new("#?.?? 2,1".parse().unwrap());
    }

    fn check_expected_combination_count(s: &str) {
        let line: Line = s.parse().unwrap();
        let count = line.count_combinations();
        let mut comb_iter = LineCombinationIter::new(line);
        let mut iter_count = 0usize;
        while comb_iter.combination_digit.is_some() {
            iter_count += 1;
            // println!("{:?}", comb_iter.line.marks);
            comb_iter.advance();
        }
        assert_eq!(count, iter_count, "{s}");
    }

    #[test]
    fn test_expected_combination_count() {
        check_expected_combination_count("?? 1");
        check_expected_combination_count("? 1");
        check_expected_combination_count("#?.?? 2,1");
        check_expected_combination_count("???.### 1,1,3");
        check_expected_combination_count(".??..??...?##. 1,1,3");
        check_expected_combination_count("?#?#?#?#?#?#?#? 1,3,1,6");
    }

    fn check_valid_combination_count(s: &str, expected: usize) {
        let line: Line = s.parse().unwrap();
        let mut line: Line = s.parse().unwrap();
        // println!("Before: {line:?}");
        line.reduce_left();
        // println!("After: {line:?}");
        line.reduce_right();
        let mut comb_iter = LineCombinationIter::new(line);
        let mut iter_count = 0usize;
        while comb_iter.combination_digit.is_some() {
            if comb_iter.line.valid_candidate().expect(&format!("Candidate generation issue: {:?} {:?}", comb_iter.line, s)) {
                iter_count += 1;
                // println!("Valid line: {:?}", comb_iter.line);
            } else {
                // println!("Invalid line: {:?}", comb_iter.line);
            }
            comb_iter.advance();
        }
        assert_eq!(expected, iter_count, "{s:?}");
    }

    #[test]
    fn test_valid_combination_count() {
        check_valid_combination_count("?? 1", 2);
        check_valid_combination_count("? 1", 1);
        check_valid_combination_count("#?.?? 2,1", 2);
        check_valid_combination_count("???.### 1,1,3", 1);
        check_valid_combination_count(".??..??...?##. 1,1,3", 4);
        check_valid_combination_count("?#?#?#?#?#?#?#? 1,3,1,6", 1);
        check_valid_combination_count("????.#...#... 4,1,1", 1);
        check_valid_combination_count("????.######..#####. 1,6,5", 4);
        check_valid_combination_count("?###???????? 3,2,1", 10);
        check_valid_combination_count("?#?.#?##???? 3,1,4", 1);
    }
}
