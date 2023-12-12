pub fn extended_euclidean(a: i64, b: i64) -> (i64, i64) {
    assert!(a > 0);
    assert!(b > 0);
    let mut big = a;
    let mut small = b;
    let mut big_x = 1i64;
    let mut small_x = 0i64;
    let mut big_y = 0i64;
    let mut small_y = 1i64;

    while small != 0 {
        let q = big / small;
        let r = big % small;
        big = small;
        small = r;
        let next_x = big_x - q * small_x;
        big_x = small_x;
        small_x = next_x;
        let next_y = big_y - q * small_y;
        big_y = small_y;
        small_y = next_y;
    }
    (big_x, big_y)
}

pub fn chinese_remainder_theorem((n_a, r_a): (i64, i64), (n_b, r_b): (i64, i64)) -> u64 {
    let (x, y) = extended_euclidean(r_a, r_b);
    (n_b * r_a * x + n_a * r_b * y).rem_euclid(r_a * r_b) as u64
}

pub fn count_combinations(n: u64, r: u64) -> u64 {
    // From Stack Overflow
    if r > n {
        0
    } else {
        // Only one division at the end might be faster, but also more likely to overflow
        (1..=r.min(n - r)).fold(1, |acc, val| acc * (n - val + 1) / val)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn check_pair(a: i64, b: i64, gcd: i64) {
        let (x, y) = extended_euclidean(a, b);
        // println!("{a} {b} {x} {y}");
        assert_eq!(x * a + y * b, gcd);
    }

    #[test]
    fn test_euclid_coprime() {
        check_pair(3, 2, 1);
        check_pair(2, 3 ,1);

        check_pair(33, 20, 1);
        check_pair(20, 33, 1);
    }

    #[test]
    fn test_euclid_example() {
        check_pair(240, 46, 2);
        check_pair(46, 240, 2);
    }

    #[test]
    fn test_chinese_remainder_theorem_example() {
        assert_eq!(chinese_remainder_theorem((0, 3), (3, 4)), 3);
    }

    #[test]
    fn test_count_combinations() {
        assert_eq!(count_combinations(3, 0), 1);
        assert_eq!(count_combinations(3, 1), 3);
        assert_eq!(count_combinations(5, 2), 10);
        assert_eq!(count_combinations(5, 3), 10);
        assert_eq!(count_combinations(0, 0), 1);
        assert_eq!(count_combinations(1, 2), 0);
    }
}