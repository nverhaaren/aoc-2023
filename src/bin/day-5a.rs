use std::collections::{BTreeMap, HashSet};
use std::io;
use std::io::{BufRead, BufReader};
use regex::Regex;

fn parse_number(s: &str) -> u64 {
    s.parse().expect(&format!("number parse issue {s:?}"))
}

fn get_seeds(mut lines: impl Iterator<Item=String>) -> Vec<u64> {
    let re = Regex::new(r"\d+").unwrap();
    let first = lines.next().expect("Empty input");
    assert!(first.starts_with("seeds: "), "Did not find seeds: line");
    assert!(lines.next().expect("Only one line").is_empty(), "Expected empty line");
    re.find_iter(&first)
        .map(|m| parse_number(m.as_str()))
        .collect()
}

fn build_map(map_name: &str, mut lines: impl Iterator<Item=String>) -> BTreeMap<u64, (u64, u64)> {
    let re = Regex::new(r"^(\d+) (\d+) (\d+)$").unwrap();
    assert_eq!(lines.next().expect("Unexpected empty line"), format!("{map_name} map:"));
    lines
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let (_, [dest, source, length]) = re.captures(&line)
                .expect("Map format was not as expected").extract();
            (parse_number(source), (parse_number(dest), parse_number(length)))
        })
        .collect()
}

fn lookup(map: &BTreeMap<u64, (u64, u64)>, input: u64) -> u64 {
    let Some((source, (dest, length))) = map.range(..=input).last() else {
        return input;
    };
    let offset = input.checked_sub(*source).expect("tree lookup logic error");
    if offset < *length {
        *dest + offset
    } else {
        input
    }
}

fn process_lines(mut lines: impl Iterator<Item=String>) -> u64 {
    let seeds = get_seeds(&mut lines);
    let seed_to_soil = build_map("seed-to-soil", &mut lines);
    let soil_to_fertilizer = build_map("soil-to-fertilizer", &mut lines);
    let fertilizer_to_water = build_map("fertilizer-to-water", &mut lines);
    let water_to_light = build_map("water-to-light", &mut lines);
    let light_to_temperature = build_map("light-to-temperature", &mut lines);
    let temperature_to_humidity = build_map("temperature-to-humidity", &mut lines);
    let humidity_to_location = build_map("humidity-to-location", &mut lines);

    seeds.iter().copied()
        .map(|x| lookup(&seed_to_soil, x))
        // .inspect(|x| println!("{x}"))
        .map(|x| lookup(&soil_to_fertilizer, x))
        .map(|x| lookup(&fertilizer_to_water, x))
        .map(|x| lookup(&water_to_light, x))
        .map(|x| lookup(&light_to_temperature, x))
        .map(|x| lookup(&temperature_to_humidity, x))
        .map(|x| lookup(&humidity_to_location, x))
        .min().expect("no seeds")
}

fn main() {
    let stdin = io::stdin();
    let reader = BufReader::with_capacity(256, stdin.lock());
    println!("{}", process_lines(reader.lines()
        .map(|s| s.expect("unicode issue"))))
}
