mod solution {
    #[expect(clippy::unreadable_literal)]
    pub const I32_POW_OF_10: [i32; 10] = [
        1,
        10,
        100,
        1000,
        10000,
        100000,
        1000000,
        10000000,
        100000000,
        1000000000,
    ];

    #[expect(clippy::unreadable_literal)]
    pub const I32_POW_OF_10_PREFIX_SUM: [i32; 10] = [
        1,
        11,
        111,
        1111,
        11111,
        111111,
        1111111,
        11111111,
        111111111,
        1111111111,
    ];

    pub fn main(n: i32, k: i32) -> i32 {
        self::search_root(n, k - 1)
    }

    fn search_root(n: i32, mut k: i32) -> i32 {
        for i in 1..=9 {
            match self::search(n, k, i) {
                Ok(out) => return out,
                Err(num_covered) => k -= num_covered,
            }
        }
        panic!("?!?")
    }

    /// Search for the `k+1`-th integer in lexicographical order
    /// within the range `1..=n` and has the particular `prefix`.
    /// 
    /// If found, return `Ok` of the found integer.
    /// If not found, return number of integers that are covered.
    #[expect(clippy::match_wild_err_arm)]
    #[expect(clippy::panic_in_result_fn)]
    fn search(
        n: i32,
        k: i32,
        prefix: i32) -> Result<i32, i32> {
        debug_assert!(n >= 1);
        debug_assert!(k >= 0);
        debug_assert!(prefix >= 1);
        if k == 0 { return Ok(prefix); }

        let m = count_with_prefix(n, prefix);
        if k >= m { return Err(m); }

        // now `k` is in range `1..m`.
        let mut acc_covered = 1; // one for `prefix` itself.
        for i in 0..=9 {
            let m = count_with_prefix(n, prefix * 10 + i);
            if acc_covered + m > k {
                match search(n, k - acc_covered, prefix * 10 + i) {
                    Ok(out) => return Ok(out),
                    Err(_) => panic!("?!?"),
                }
            } else {
                acc_covered += m;
            }
        }
        panic!("?!?")
    }

    /// Count integers within the range `1..=n` and has the particular `prefix`.
    pub fn count_with_prefix(n: i32, prefix: i32) -> i32 {
        debug_assert!(n >= 1);
        debug_assert!(prefix >= 1);

        let n_digits = num_digits_of(n);
        let prefix_digits = num_digits_of(prefix);
        debug_assert!(n_digits >= prefix_digits);

        let suffix_digits = n_digits - prefix_digits;
        if  suffix_digits == 0 {
            return i32::from(n >= prefix)
        }

        use core::cmp::Ordering::{Less, Equal, Greater};
        match prefix.cmp(&(n / I32_POW_OF_10[suffix_digits])) {
            Less    => I32_POW_OF_10_PREFIX_SUM[suffix_digits],
            Greater => I32_POW_OF_10_PREFIX_SUM[suffix_digits - 1],
            Equal => 1 + {
                (0..=9)
                    .map(|i| self::count_with_prefix(n, prefix * 10 + i))
                    .sum::<i32>()
            },
        }
    }

    pub fn is_prefix_of(n: i32, prefix: i32) -> bool {
        debug_assert!(n >= 1);
        debug_assert!(prefix >= 1);
        I32_POW_OF_10
            .iter()
            .any(|&pow_of_10| n / pow_of_10 == prefix)
    }

    fn num_digits_of(n: i32) -> usize {
        debug_assert!(n >= 1);
        (1..=9)
            .filter(|&i| I32_POW_OF_10[i] > n)
            .min()
            .unwrap_or(10) as _
    }
}

fn main() {
    for (n, prefix) in [
        (288, 1),
        (288, 2),
        (288, 3),
        (2888, 1),
        (2888, 2),
        (2888, 3),
    ] {
        assert_eq!(
            solution::count_with_prefix(n, prefix),
            (1..=n)
                .filter(|&i| solution::is_prefix_of(i, prefix))
                .count() as _,
            "test case n: {n}, prefix: {prefix}");
    }

    assert_eq!(solution::main(13, 2), 10);
    assert_eq!(solution::main(1, 1), 1);

    for (n, k) in [
        (288, 1),
        (288, 10),
        (288, 100),
        (2888, 1),
        (2888, 10),
        (2888, 100),
    ] {
        let mut vec = (1..=n).collect::<Vec<_>>();
        vec.sort_unstable_by_key(i32::to_string);
        assert_eq!(
            solution::main(n, k),
            vec[k as usize - 1],
            "test case n: {n}, k: {k}");
    }
}
