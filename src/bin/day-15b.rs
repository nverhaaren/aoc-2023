use std::collections::HashMap;
use std::io;
use std::io::{BufRead, Read};
use itertools::Itertools;

fn main() {
    let mut stdin = io::stdin();
    let mut input = vec![];
    stdin.read_to_end(&mut input).expect("Input issue");
    // println!("Input: {}", str::from_utf8(input.as_slice()).unwrap());
    assert_eq!(input.pop(), Some('\n' as u8));
    if input.last().copied() == Some('\r' as u8) {  // Oh windows
        input.pop();
    }
    let result = process_splits(input.split(|c| *c == (',' as u8)));
    println!("{result}");
}

fn hash(buf: &[u8]) -> u8 {
    // println!("buf len: {}", buf.len());
    let mut result = 0u32;
    for c in buf.iter().copied() {
        result += c as u32;
        result *= 17;
        result %= 256;
    }
    assert!(result < 256, "{result}");
    // println!("{} -> {}", str::from_utf8(buf).unwrap(), result);
    result as u8
}

struct AocHashMap {
    buckets: [Vec<(Vec<u8>, u8)>; 256]
}

fn process_splits<'a>(it: impl Iterator<Item=&'a[u8]>) -> usize {
    let mut map = AocHashMap::new();
    for op in it {
        map.apply_operation(op);
    }
    map.focusing_power().values().sum()
}

impl AocHashMap {
    pub const fn new() -> Self {
        const BUCKET: Vec<(Vec<u8>, u8)> = vec![];
        Self { buckets: [BUCKET; 256] }
    }

    pub fn apply_operation(&mut self, op: &[u8]) {
        let (last, rest) = op.split_last().expect("Invalid operation");
        match *last as char {
            '-' => self.remove(rest),
            power => {
                let (eq, key) = rest.split_last().expect("Invalid operation");
                assert_eq!(*eq, '=' as u8);
                self.update(key, power.to_digit(10).expect("Not numeric") as u8);
            },
        }
    }

    fn remove(&mut self, key: &[u8]) {
        let hash = hash(key);
        let bucket = &mut self.buckets[hash as usize];
        bucket.iter()
            .find_position(|(k, _power)| {
                k.as_slice() == key
            })
            .map(|(idx, _)| idx)
            .map(|idx| bucket.remove(idx));
    }

    fn update(&mut self, key: &[u8], power: u8) {
        let hash = hash(key);
        let bucket = &mut self.buckets[hash as usize];
        bucket.iter_mut()
            .find(|(k, _power)| {
                k.as_slice() == key
            })
            .map(|(_k, bucket_power)| {*bucket_power = power})
            .unwrap_or_else(|| bucket.push((key.to_owned(), power)));
    }

    pub fn focusing_power(&self) -> HashMap<Vec<u8>, usize> {
        let mut result = HashMap::new();
        for (bucket_idx, bucket) in self.buckets.iter().enumerate() {
            for (slot_idx, (key, length)) in bucket.iter().enumerate() {
                *result.entry(key.clone()).or_default() += (bucket_idx + 1) * (slot_idx + 1)
                    * (*length as usize);
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hash() {
        assert_eq!(hash("HASH".as_bytes()), 52);
    }
}
