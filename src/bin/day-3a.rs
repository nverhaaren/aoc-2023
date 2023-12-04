use std::collections::{HashMap, HashSet};
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
    s.parse().expect("number parse issue")
}

fn process_lines(lines: impl Iterator<Item=String>) -> u32 {
    let all_lines: Vec<_> = lines.collect();
    let re = Regex::new(r"([^A-Za-z0-9.])|(\d+)").unwrap();
    let parsed_lines: Vec<(Vec<(usize, &str)>, Vec<usize>)> = all_lines.iter()
        .map(|line| {
            let mut numbers = vec![];
            let mut symbols = vec![];
            for captures in re.captures_iter(line) {
                if let Some(s) = captures.get(1) {
                    numbers.push((substr_index(line.as_str(), s.as_str()), s.as_str()));
                } else {
                    let s = captures.get(2).expect("regex logic error");
                    symbols.push(substr_index(line.as_str(), s.as_str()));
                }
            }
            (numbers, symbols)
        })
        .collect();
    let _ = (0..parsed_lines.len())
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
        .map(|(symbols, same_numbers, adj_numbers0, adj_numbers1)| {
            symbols.iter()
                .flat_map(move |&symbol_idx| same_numbers.iter()
                    .filter_map(move |(offset, slice)| -> Option<u32> {
                        if *offset == symbol_idx + 1 || *offset + slice.len() == symbol_idx - 1 {
                            Some(parse_number(slice))
                        } else {
                            None
                        }
                    })
                    .chain([0u32].iter().copied())
                    .chain([adj_numbers0, adj_numbers1].iter().map(|x| 0u32))
                    // .chain(adj_numbers.iter()
                    //            .map(|x| 0u32)
                        // .map(|v| v.iter().as_slice())
                        // .flat_map(|x| [0u32].iter().copied())
                    // )
                )
                // .flatten()
        });
        // .flatten()
        // .collect::<HashSet<_>>()
        // .iter().sum::<u32>()
    0
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