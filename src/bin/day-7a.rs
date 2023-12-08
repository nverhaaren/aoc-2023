use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::io;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use anyhow::anyhow;
use itertools::Itertools;
use regex::Regex;

fn parse_number(s: &str) -> u64 {
    s.parse().expect(&format!("number parse issue {s:?}"))
}

trait InspectVal: Sized {
    fn inspect_val(self, f: impl FnOnce(&Self) -> ()) -> Self {
        f(&self);
        self
    }

    fn inspect_val_mut(mut self, f: impl FnOnce(&mut Self) -> ()) -> Self {
        f(&mut self);
        self
    }
}

impl<T> InspectVal for T {}

////////////

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, Hash)]
struct Label(u8);

impl Label {
    pub fn new(c: char) -> Self {
        c.try_into().expect(&format!("No label {c}"))
    }
}

impl TryFrom<char> for Label {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' | 'K' | 'Q' | 'J' | 'T' | '2'..='9' => Ok(Self(value as u8)),
            _ => Err(value),
        }
    }
}

impl PartialOrd for Label {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            return Some(Ordering::Equal);
        }
        Some(match (self.0 as char, other.0 as char) {
            ('A', _) => Ordering::Greater,
            (_, 'A') => Ordering::Less,
            ('K', _) => Ordering::Greater,
            (_, 'K') => Ordering::Less,
            ('Q', _) => Ordering::Greater,
            (_, 'Q') => Ordering::Less,
            ('J', _) => Ordering::Greater,
            (_, 'J') => Ordering::Less,
            ('T', _) => Ordering::Greater,
            (_, 'T') => Ordering::Less,
            (s, o) => s.cmp(&o),
        })
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, Hash)]
struct Hand {
    labels: [Label; 5],
    hand_type: HandType,
}

impl Hand {
    pub fn new(labels: [Label; 5]) -> Self {
        let hand_type = Self::hand_type(&labels);
        Self { labels, hand_type }
    }

    fn hand_type(labels: &[Label; 5]) -> HandType {
        let mut groups = HashMap::new();
        let mut max_val = 0u8;
        for label in labels {
            let val = groups.entry(*label).or_insert(0u8);
            *val += 1;
            max_val = max_val.max(*val);
        }
        match groups.len() {
            1 => HandType::FiveOfAKind,
            2 => match max_val {
                4 => HandType::FourOfAKind, 3 => HandType::FullHouse, _ => unreachable!()
            },
            5 => HandType::HighCard,
            4 => HandType::OnePair,
            3 => match max_val {
                3 => HandType::ThreeOfAKind, 2 => HandType::TwoPair, _ => unreachable!()
            },
            _ => unreachable!(),
        }
    }
}

impl FromStr for Hand {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // It feels like it should be possible to do this without dynamic allocation, but rust is
        // not making it easy.
        // https://internals.rust-lang.org/t/collecting-iterators-into-arrays/10330
        let labels: Vec<_> = s.chars()
            .map(|c| Label::try_from(c))
            .try_collect().map_err(|c| anyhow!("Char {c} is not valid in a hand"))?;
        let labels: [Label; 5] = labels.try_into().map_err(|v| anyhow!("Hand length not 5: {v:?}"))?;
        Ok(Self::new(labels))
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        (self.hand_type, self.labels).partial_cmp(&(other.hand_type, other.labels))
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn process_lines(lines: impl Iterator<Item=String>) -> u64 {
    let re = Regex::new(r"([2-9AKQJT]{5}) (\d+)").unwrap();
    let mut pairs: Vec<(Hand, u64)> = lines
        .map(|line| {
            let (_, [hand, bid]) = re.captures(&line)
                .expect("Line did not match pattern").extract();
            (hand.parse().unwrap(), parse_number(bid))
        })
        .collect();
    pairs.sort();
    pairs.iter()
        .enumerate()
        .map(|(idx, (_, bid))| (idx as u64 + 1) * bid)
        .sum()
}

fn main() {
    let stdin = io::stdin();
    let reader = BufReader::with_capacity(256, stdin.lock());
    println!("{}", process_lines(reader.lines()
        .map(|s| s.expect("unicode issue"))))
}
