use std::{io, mem};
use std::collections::HashSet;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use anyhow::{anyhow, bail};
use itertools::Itertools;
use regex::Regex;

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
    pub const fn empty() -> Self {
        Self { marks: vec![], seqs: vec![] }
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
                .map_err(|e| anyhow!("Could not parse int"))?
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
                            assert_eq!(self.seqs[seq_idx], broken_group, "{self:?}");
                            seq_idx += 1;
                            broken_group = 0;
                        }
                        mark_idx += 1;
                        continue;
                    },
                    Mark::Broken => broken_group += 1,
                    Mark::Unknown => if broken_group > 0 {
                        if self.seqs[seq_idx] < broken_group {
                            broken_group += 1;
                            mark_idx += 1;
                            continue;
                        }
                        assert_eq!(self.seqs[seq_idx], broken_group);
                        broken_group = 0;
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

            let unknown_broken_len = (mark_idx..self.marks.len()).into_iter()
                .take_while(|idx| self.marks[*idx] == Mark::Unknown || self.marks[*idx] == Mark::Broken)
                .count();

            if unknown_broken_len == self.seqs[seq_idx] {
                mark_idx += unknown_broken_len;
                broken_group = unknown_broken_len;
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
}

fn process_lines(lines: impl Iterator<Item=String>) -> usize {
    todo!()
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
}
