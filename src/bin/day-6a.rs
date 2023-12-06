use std::collections::{BTreeMap, HashSet};
use itertools::Itertools;

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

fn calc_distance(hold: u64, total: u64) -> u64 {
    hold * total.checked_sub(hold).expect("invalid strategy")
}

fn count_solutions(time: u64, distance: u64) -> u64 {
    (0u64..=time).into_iter()
        .map(|hold| calc_distance(hold, time))
        .filter(|dist| *dist > distance)
        .count() as u64
}

fn main() {
    let sample = [(7u64, 9u64), (15, 40), (30, 200)];

    let sample_result: u64 = sample.iter()
        .map(|(t, d)| count_solutions(*t, *d))
        .product();
    println!("Sample: {sample_result}");

    let actual = [(49u64, 298u64), (78, 1185), (79, 1066), (80, 1181)];

    let actual_result: u64 = actual.iter()
        .map(|(t, d)| count_solutions(*t, *d))
        .product();
    println!("Actual: {actual_result}");
}
