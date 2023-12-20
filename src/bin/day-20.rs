use std::{io, iter, str};
use std::collections::{HashMap, VecDeque};
use std::hash::Hash;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use anyhow::{anyhow, bail};
use itertools::Itertools;

fn main() {
    let stdin = io::stdin();
    let reader = BufReader::with_capacity(256, stdin.lock());
    let lines: Vec<_> = reader.lines()
        .try_collect().expect("Unicode issue");
    let module_specs: Vec<ModuleSpec> = lines.iter()
        .map(|x| x.parse())
        .try_collect().expect("parse issue");

    println!("First part: {}", part_1(&module_specs));

    // println!("First part: {result} ({elapsed:.2?})");
    // println!("Second part: {}", part_2(&map));
}

fn part_1(specs: &[ModuleSpec]) -> usize {
    let name_idx: HashMap<_, _> = specs.iter().enumerate()
        .map(|(idx, spec)| (spec.name.to_owned(), idx))
        .collect();

    let mut flip_flop_idx = 0usize;
    let mut conjunction_idx = 0usize;
    let modules: Vec<_> = specs.iter()
        .map(|spec| {
            Module {
                kind: spec.kind,
                state_idx: match spec.kind {
                    ModuleKind::Broadcaster => 0,
                    ModuleKind::FlipFlop => {
                        let result = flip_flop_idx;
                        flip_flop_idx += 1;
                        result
                    },
                    ModuleKind::Conjunction => {
                        let result = conjunction_idx;
                        conjunction_idx += 1;
                        result
                    },
                },
                destinations: spec.destinations.iter()
                    .map(|x| name_idx.get(x).copied().unwrap())
                    .collect()
            }
        })
        .collect();
    let mut input_maps: Vec<_> = iter::repeat_with(|| HashMap::new())
        .take(conjunction_idx)
        .collect();
    for (idx, m) in modules.iter().enumerate() {
        for dest in m.destinations.iter().copied() {
            let dest = &modules[dest];
            if dest.kind != ModuleKind::Conjunction { continue; }
            let map = &mut input_maps[dest.state_idx];
            map.insert(idx, map.len());
        }
    };
    let mut full_state = FullState {
        flip_flops: vec![false; flip_flop_idx],
        conjunctions: input_maps.iter()
            .map(|m| vec![Signal::Low; m.len()])
            .collect(),
    };
    let broadcaster_idx = name_idx.get("broadcaster").copied().expect("no broadcaster");

    let (low_sent, high_sent) = push_button(
        &mut full_state, &modules, &input_maps, broadcaster_idx);
    low_sent * high_sent
}

fn push_button(full_state: &mut FullState, modules: &[Module],
               input_maps: &Vec<HashMap<usize, usize>>,
               broadcaster_idx: usize) -> (usize, usize) {
    let empty_map: HashMap<usize, usize> = HashMap::new();

    let mut operations = VecDeque::new();
    operations.push_back((
        usize::MAX,  // ignored
        broadcaster_idx,
        Signal::Low,
    ));

    let mut low_sent = 1usize;  // for broadcaster
    let mut high_sent = 0usize;
    while let Some((source_idx, idx, signal)) = operations.pop_front() {
        let module = &modules[idx];
        let source_map = match module.kind() {
            ModuleKind::Conjunction => &input_maps[module.state_idx],
            _ => &empty_map,
        };
        let (low, high) = full_state.process(module, idx,signal, source_idx, source_map, &mut operations);
        low_sent += low;
        high_sent += high;
    };
    (low_sent, high_sent)
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
enum ModuleKind {
    Broadcaster,
    FlipFlop,
    Conjunction,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
struct ModuleSpec {
    name: String,
    kind: ModuleKind,
    destinations: Vec<String>,
}

impl FromStr for ModuleSpec {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(' ');
        let core = split.next().ok_or(anyhow!("empty input"))?;
        if split.next() != Some("->") {
            bail!("did not find \"->\"");
        }
        let mut destinations = vec![];
        for s in split {
            let name = s.strip_suffix(',').unwrap_or(s).to_owned();
            destinations.push(name);
        }
        match core.strip_prefix('%') {
            None => (),
            Some(name) => return Ok(Self {
                name: name.to_owned(), kind: ModuleKind::FlipFlop, destinations
            }),
        };
        match core.strip_prefix('&') {
            None => (),
            Some(name) => return Ok(Self {
                name: name.to_owned(), kind: ModuleKind::Conjunction, destinations
            }),
        };
        if core != "broadcaster" {
            bail!("Unrecognized core: {core:?}");
        }
        Ok(Self { name: core.to_owned(), kind: ModuleKind::Broadcaster, destinations})
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, Ord, PartialOrd, Default)]
enum Signal {
    #[default]
    Low,
    High
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
struct Module {
    kind: ModuleKind,
    state_idx: usize,
    destinations: Vec<usize>,
}

impl Module {
    pub fn kind(&self) -> ModuleKind { self.kind }
    pub fn state_idx(&self) -> usize { self.state_idx }
    pub fn destinations(&self) -> &[usize] { self.destinations.as_slice() }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
struct FullState {
    flip_flops: Vec<bool>,
    conjunctions: Vec<Vec<Signal>>,
}

impl FullState {
    pub fn process(&mut self, module: &Module, module_idx: usize, input: Signal, source: usize,
                   source_map: &HashMap<usize, usize>, out: &mut impl Extend<(usize, usize, Signal)>) -> (usize, usize) {
        let mut low_sent = 0usize;
        let mut high_sent = 0usize;
        match module.kind() {
            ModuleKind::Broadcaster => {
                out.extend(module.destinations().iter().copied().map(|out_idx| {
                    (module_idx, out_idx, input)
                }));
                match input {
                    Signal::Low => low_sent += module.destinations.len(),
                    Signal::High => high_sent += module.destinations.len(),
                }
            },
            ModuleKind::FlipFlop => {
                if input == Signal::High { return (low_sent, high_sent); }
                let state = &mut self.flip_flops[module.state_idx()];
                let out_signal = if *state {
                    *state = false;
                    Signal::Low
                } else {
                    *state = true;
                    Signal::High
                };
                match out_signal {
                    Signal::Low => low_sent += module.destinations.len(),
                    Signal::High => high_sent += module.destinations.len(),
                }
                out.extend(module.destinations.iter().copied().map(|out_idx| {
                    (module_idx, out_idx, out_signal)
                }));
            }
            ModuleKind::Conjunction => {
                self.conjunctions[module.state_idx][*source_map.get(&source).expect("source map issue")] = input;
                let out_signal = if self.conjunctions[module.state_idx].iter().copied().all(|x| x == Signal::High) {
                    Signal::Low
                } else {
                    Signal::High
                };
                match out_signal {
                    Signal::Low => low_sent += module.destinations.len(),
                    Signal::High => high_sent += module.destinations.len(),
                }
                out.extend(module.destinations.iter().copied().map(|out_idx| {
                    (module_idx, out_idx, out_signal)
                }));
            }
        }
        (low_sent, high_sent)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_module_spec_a() {
        let ms: ModuleSpec = "broadcaster -> a, b, c".parse().unwrap();
        assert_eq!(ModuleSpec {
            name: "broadcaster".to_owned(),
            kind: ModuleKind::Broadcaster,
            destinations: ["a", "b", "c"].iter().copied().map(|s| s.to_owned()).collect(),
        }, ms);
    }

    #[test]
    fn test_parse_module_spec_b() {
        let ms: ModuleSpec = "%c -> inv".parse().unwrap();
        assert_eq!(ModuleSpec {
            name: "c".to_owned(),
            kind: ModuleKind::FlipFlop,
            destinations: ["inv"].iter().copied().map(|s| s.to_owned()).collect(),
        }, ms);
    }

    #[test]
    fn test_parse_module_spec_c() {
        let ms: ModuleSpec = "&inv -> a".parse().unwrap();
        assert_eq!(ModuleSpec {
            name: "inv".to_owned(),
            kind: ModuleKind::Conjunction,
            destinations: ["a"].iter().copied().map(|s| s.to_owned()).collect(),
        }, ms);
    }
}
