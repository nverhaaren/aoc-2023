use std::io;
use std::io::{Read};

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

fn process_splits<'a>(it: impl Iterator<Item=&'a[u8]>) -> u64 {
    it
        // .inspect(|buf| println!("{}", str::from_utf8(buf).unwrap()))
        .map(hash)
        .map(|x| x as u64)
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hash() {
        assert_eq!(hash("HASH".as_bytes()), 52);
    }
}
