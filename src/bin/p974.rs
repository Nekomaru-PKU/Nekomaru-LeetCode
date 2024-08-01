mod solution {
    use super::*;

    pub fn main(nums: Vec<i32>, k: i32) -> i32 {
        use fast_hash::FastHashMap as HashMap;

        // we define `map[sum]` as the count of `m` that satisfies
        // `nums[0..m].sum() % k == sum` where m is in [0, n].
        let mut map = HashMap::<i32, _>::default();
        map.reserve(nums.len());
        // trivially, `nums[0..0].sum() == 0`.
        map.insert(0, 1);

        let mut sum = 0;
        for num in nums {
            // here we update `sum` as `nums[0..=i].sum() % k` and
            // make sure that `sum` is non-negative.
            sum = (sum + k + num % k) % k;
            // then we increase the count of `sum`.
            map.entry(sum).and_modify(|n| *n += 1).or_insert(1);
        }

        map.values().map(|n| n * (n - 1) / 2).sum()
    }
}

mod fast_hash {
    use std::{
        collections::HashMap,
        hash::Hasher,
        hash::BuildHasherDefault,
        marker::PhantomData,
    };

    #[derive(Default, Clone, Copy)]
    pub struct FastHasher<T>(u64, PhantomData<T>);

    impl Hasher for FastHasher<i32> {
        fn write_i32(&mut self, n: i32) {
            self.0 = n as _;
        }

        fn write(&mut self, _: &[u8]) {
            panic!("invalid use.")
        }

        fn finish(&self) -> u64 { self.0 }
    }

    pub type FastHashMap<K, V> = HashMap<K, V, BuildHasherDefault<FastHasher<K>>>;
}

use leetcode::perf::print_time;

fn main() {
    assert_eq!(solution::main(vec![4, 5, 0, -2, -3, 1], 5), 7);
    assert_eq!(solution::main(vec![5], 9), 0);

    // the worst case of nums.length = 3 * 10^4,
    // and sum of every subarray is a multiple of k.
    let vec = {
        let mut vec = Vec::new();
        vec.resize(3000, -300);
        vec
    };
    let res = print_time("perf", || solution::main(vec, 3));
    // the result should be C(3001, 2) = 4501500
    assert_eq!(res, 4501500);

    // the worst case of nums.length = 3 * 10^4,
    // and sum of every subarray is different mod k.
    let vec = {
        let mut vec = Vec::new();
        vec.resize(3000, -1);
        vec
    };
    let res = print_time("perf", || solution::main(vec, 3001));
    assert_eq!(res, 0);
}
