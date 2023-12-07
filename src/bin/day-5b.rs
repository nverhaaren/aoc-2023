use std::collections::{BTreeMap, HashSet};
use std::io;
use std::io::{BufRead, BufReader};
use regex::Regex;

fn parse_number(s: &str) -> u64 {
    s.parse().expect(&format!("number parse issue {s:?}"))
}

fn get_seeds(mut lines: impl Iterator<Item=String>) -> Vec<(u64, u64)> {
    let re = Regex::new(r"(\d+) (\d+)").unwrap();
    let first = lines.next().expect("Empty input");
    assert!(first.starts_with("seeds: "), "Did not find seeds: line");
    assert!(lines.next().expect("Only one line").is_empty(), "Expected empty line");
    re.captures_iter(&first)
        .map(|m| {
            let (_, [start, length]) = m.extract();
            (parse_number(start), parse_number(length))
        })
        .collect()
}

fn build_maps(map_name: &str, mut lines: impl Iterator<Item=String>)
    -> (BTreeMap<u64, (u64, u64)>, BTreeMap<u64, (u64, u64)>)
{
    let re = Regex::new(r"^(\d+) (\d+) (\d+)$").unwrap();
    assert_eq!(lines.next().expect("Unexpected empty line"), format!("{map_name} map:"));
    let forward: BTreeMap<u64, (u64, u64)> = lines
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let (_, [dest, source, length]) = re.captures(&line)
                .expect("Map format was not as expected").extract();
            (parse_number(source), (parse_number(dest), parse_number(length)))
        })
        .collect();
    let backward = forward.iter()
        .map(|(source, (dest, length))| (*dest, (*source, *length)))
        .collect();
    (forward, backward)
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

fn edges(map: &BTreeMap<u64, (u64, u64)>) -> impl Iterator<Item=u64> + '_ {
    map.iter()
        .map(|(source, (_dest, length))| [*source, source + length])
        .flatten()
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

fn process_lines(mut lines: impl Iterator<Item=String>) -> u64 {
    let seeds = get_seeds(&mut lines);
    let seed_to_soil = build_maps("seed-to-soil", &mut lines);
    let soil_to_fertilizer = build_maps("soil-to-fertilizer", &mut lines);
    let fertilizer_to_water = build_maps("fertilizer-to-water", &mut lines);
    let water_to_light = build_maps("water-to-light", &mut lines);
    let light_to_temperature = build_maps("light-to-temperature", &mut lines);
    let temperature_to_humidity = build_maps("temperature-to-humidity", &mut lines);
    let humidity_to_location = build_maps("humidity-to-location", &mut lines);

    let seed_ranges: Vec<_> = seeds.iter().copied()
        .map(|(start, length)| start..(start + length))
        .collect();

    edges(&humidity_to_location.0)
        .map(|x| lookup(&temperature_to_humidity.1, x))
        .chain(edges(&temperature_to_humidity.0))
        .map(|x| lookup(&light_to_temperature.1, x))
        .chain(edges(&light_to_temperature.0))
        .map(|x| lookup(&water_to_light.1, x))
        .chain(edges(&water_to_light.0))
        .map(|x| lookup(&fertilizer_to_water.1, x))
        .chain(edges(&fertilizer_to_water.0))
        .map(|x| lookup(&soil_to_fertilizer.1, x))
        .chain(edges(&soil_to_fertilizer.0))
        .map(|x| lookup(&seed_to_soil.1, x))
        .chain(edges(&seed_to_soil.0))
        .chain(seeds.iter().map(|&v| v.0))
        .collect::<HashSet<_>>().iter()
        .filter(|x| seed_ranges.iter().any(|r| r.contains(x))).copied()
        .collect::<Vec<_>>()
        .inspect_val_mut(|v| {
            v.sort();
            println!("{v:?}");
        }).iter().copied()
        .map(|x| lookup(&seed_to_soil.0, x))
        .map(|x| lookup(&soil_to_fertilizer.0, x))
        .map(|x| lookup(&fertilizer_to_water.0, x))
        .map(|x| lookup(&water_to_light.0, x))
        .map(|x| lookup(&light_to_temperature.0, x))
        .map(|x| lookup(&temperature_to_humidity.0, x))
        .map(|x| lookup(&humidity_to_location.0, x))
        .min().expect("no seeds")
}

fn main() {
    let stdin = io::stdin();
    let reader = BufReader::with_capacity(256, stdin.lock());
    println!("{}", process_lines(reader.lines()
        .map(|s| s.expect("unicode issue"))))
}
