use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::io;
use std::io::{BufRead, BufReader};
use regex::Regex;

fn substr_index(string: &str, sub: &str) -> usize {
    // Assumes strings are ascii
    let start = string.as_bytes().as_ptr();
    let sub_start = sub.as_bytes().as_ptr();
    let diff = sub_start as isize - start as isize;
    assert!(diff >= 0);
    diff as usize
}

fn parse_number(s: &str) -> u32 {
    s.parse().expect(&format!("number parse issue {s:?}"))
}

#[derive(Copy, Clone, Debug, Eq)]
struct Wrapper<'a> {
    s: &'a str,
}

impl<'a> Wrapper<'a> {
    pub fn new(s: &'a str) -> Self {
        Self { s }
    }
}

impl<'a> Hash for Wrapper<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.s.as_ptr().hash(state)
    }
}

impl<'a> PartialEq for Wrapper<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.s.as_ptr() == other.s.as_ptr()
    }
}

impl<'a> From<Wrapper<'a>> for &'a str {
    fn from(value: Wrapper<'a>) -> Self {
        value.s
    }
}

fn process_lines(lines: impl Iterator<Item=String>) -> u32 {
    let all_lines: Vec<_> = lines.collect();
    let re = Regex::new(r"([^A-Za-z0-9.])|(\d+)").unwrap();
    let parsed_lines: Vec<(Vec<(usize, &str)>, Vec<usize>)> = all_lines.iter()
        .map(|line| {
            let mut numbers = vec![];
            let mut symbols = vec![];
            for captures in re.captures_iter(line) {
                if let Some(s) = captures.get(2) {
                    numbers.push((substr_index(line.as_str(), s.as_str()), s.as_str()));
                } else {
                    let s = captures.get(1).expect("regex logic error");
                    symbols.push(substr_index(line.as_str(), s.as_str()));
                }
            }
            (numbers, symbols)
        })
        .collect();
    // println!("{parsed_lines:?}");
    (0..parsed_lines.len())
        .map(|idx| {
            let (same_numbers, symbols) = &parsed_lines[idx];
            let result: (&[usize], &[(usize, &str)], &[(usize, &str)], &[(usize, &str)]) = if idx == 0 {
                (symbols.as_slice(), same_numbers.as_slice(), parsed_lines[1].0.as_slice(), &[])
            } else if idx == parsed_lines.len() - 1 {
                (symbols.as_slice(), same_numbers.as_slice(), parsed_lines[parsed_lines.len() - 2].0.as_slice(), &[])
            } else {
                (symbols.as_slice(), same_numbers.as_slice(), parsed_lines[idx - 1].0.as_slice(), parsed_lines[idx + 1].0.as_slice())
            };
            result
        })
        .flat_map(|(symbols, same_numbers, adj_numbers0, adj_numbers1)| {
            symbols.iter()
                .flat_map(move |&symbol_idx| same_numbers.iter()
                    .filter_map(move |(offset, slice)| {
                        if *offset == symbol_idx + 1 || *offset + slice.len() == symbol_idx {
                            Some(slice)
                        } else {
                            None
                        }
                    })
                    .chain([adj_numbers0, adj_numbers1].iter()
                        .map(|x| x.iter())
                        .flatten()
                        .filter_map(|(offset, slice)| {
                            if symbol_idx >= offset.saturating_sub(1) && symbol_idx <= offset + slice.len() {
                                Some(slice)
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>()  // TODO: see about avoiding this collect
                    )
                )
        })
        .map(|x| Wrapper::new(x))
        .collect::<HashSet<_>>().iter()
        .map(|x| parse_number((*x).into()))
        // .map(|x| {println!("{x}"); x})
        .sum::<u32>()
}

fn main() {
    let stdin = io::stdin();
    let reader = BufReader::with_capacity(256, stdin.lock());
    println!("{}", process_lines(reader.lines()
        .map(|s| s.expect("unicode issue"))))
}

//Old

// fn process_lines(lines: impl Iterator<Item=String>) -> u32 {
//     let all_lines: Vec<_> = lines.collect();
//     let re = Regex::new(r"([^A-Za-z0-9.])|(\d+)").unwrap();
//     let parsed_lines: Vec<(Vec<(usize, &str)>, Vec<usize>)> = all_lines.iter()
//         .map(|line| {
//             let mut numbers = vec![];
//             let mut symbols = vec![];
//             for captures in re.captures_iter(line) {
//                 if let Some(s) = captures.get(1) {
//                     numbers.push((substr_index(line.as_str(), s.as_str()), s.as_str()));
//                 } else {
//                     let s = captures.get(2).expect("regex logic error");
//                     symbols.push(substr_index(line.as_str(), s.as_str()));
//                 }
//             }
//             (numbers, symbols)
//         })
//         .collect();
//     let _ = (0..parsed_lines.len())
//         .map(|idx| {
//             let (same_numbers, symbols) = &parsed_lines[idx];
//             let mut context = vec![];
//             if idx > 0 {
//                 context.push(parsed_lines[idx - 1].0.as_slice());
//             }
//             if idx < parsed_lines.len() - 1 {
//                 context.push(parsed_lines[idx + 1].0.as_slice());
//             }
//             (symbols.as_slice(), same_numbers.as_slice(), context)
//         })
//         .map(|(symbols, same_numbers, adj_numbers)| {
//             symbols.iter()
//                 .flat_map(move |&symbol_idx| same_numbers.iter()
//                     .filter_map(move |(offset, slice)| -> Option<u32> {
//                         if *offset == symbol_idx + 1 || *offset + slice.len() == symbol_idx - 1 {
//                             Some(parse_number(slice))
//                         } else {
//                             None
//                         }
//                     })
//                     .chain(adj_numbers.iter()
//                         .map(|v| v.iter().as_slice())
//                         .flat_map(|x| [0u32].iter().copied())
//                     )
//                 )
//             // .flatten()
//         });
//     // .flatten()
//     // .collect::<HashSet<_>>()
//     // .iter().sum::<u32>()
//     0
// }