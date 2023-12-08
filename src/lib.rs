use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::hash::Hash;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct CycleInfo<T> {
    dist_to_cycle_start: usize,
    cycle: Vec<T>,
}

impl<T> CycleInfo<T> {
    pub fn dist_to_cycle_start(&self) -> usize {
        self.dist_to_cycle_start
    }

    pub fn cycle(&self) -> &[T] {
        &self.cycle
    }
}

pub fn check_cycle<T: Hash + Eq + Clone>(it: impl IntoIterator<Item=T>) -> Option<CycleInfo<T>> {
    let mut seen = HashMap::new();
    let mut path = vec![];
    let mut dist_to_cycle_start = 0usize;
    let mut found_cycle = false;

    it.into_iter()
        .enumerate()
        .take_while(|(idx, t)| {
            let t: T = t.clone();
            let entry = seen.entry(t);
            match entry {
                Entry::Occupied(inner) => {
                    dist_to_cycle_start = *inner.get();
                    path.drain(0..dist_to_cycle_start);
                    found_cycle = true;
                    false
                },
                Entry::Vacant(inner) => {
                    path.push(inner.key().clone());
                    inner.insert(*idx);
                    true
                },
            }
        }).for_each(|_| {});
    if found_cycle {
        Some(CycleInfo { dist_to_cycle_start, cycle: path })
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use crate::check_cycle;

    #[test]
    fn test_check_simple_cycle() {
        let input = vec![0, 1, 2, 3, 4, 2];
        let cycle = check_cycle(input.iter().copied()).expect("Cycle is 2 -> 3 -> 4 -> 2");
        assert_eq!(cycle.dist_to_cycle_start(), 2);
        assert_eq!(cycle.cycle(), [2, 3, 4]);
    }
}

