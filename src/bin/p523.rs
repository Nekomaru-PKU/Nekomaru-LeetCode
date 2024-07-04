// @lc app=leetcode id=523 lang=rust
// @lc code=start

use std::hash::Hasher;

#[derive(Default, Clone, Copy)]
pub struct I32Hasher(u64);

impl Hasher for I32Hasher {
    fn write_i32(&mut self, n: i32) {
        self.0 = n as _;
    }

    fn write(&mut self, _: &[u8]) {
        panic!("invalid use of IdentityHasher.")
    }

    fn finish(&self) -> u64 { self.0 }
}

impl Solution {
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        use std::hash::BuildHasherDefault;
        use std::collections::HashMap;
        use std::collections::hash_map::Entry;
        let n = nums.len();

        // we define `map[sum]` as the minimal `m` that satisfies
        // `nums[0..m].sum() % k == sum`.
        let mut map: HashMap<i32, usize, BuildHasherDefault<I32Hasher>> =
            Default::default();
        map.reserve(n);
        // trivially, `nums[0..0].sum() == 0`.
        map.insert(0, 0);

        let mut sum = 0;
        for m in 1 ..= n {
            // here we update `sum` as `nums[0..m].sum() % k`.
            sum = (sum + nums[m - 1]) % k;
            match map.entry(sum) {
                Entry::Occupied(l) => {
                    let l = l.get();
                    // here we know that `nums[0..l].sum() % k == sum`,
                    // so together we have `nums[l..m].sum() % k == 0`.
                    // in this case, `nums[l..m]` is a a good subarray
                    // if `nums[l..m].len() >= 2`.
                    if m - l >= 2 {
                        return true;
                    }
                },
                Entry::Vacant(l) => {
                    l.insert(m);
                },
            }
        }
        false
    }
}
// @lc code=end

#[allow(clippy::bool_assert_comparison)]
fn test_case_0() {
    assert_eq!(Solution::check_subarray_sum(vec![23, 2, 4, 6, 7], 6), true);
    assert_eq!(Solution::check_subarray_sum(vec![23, 2, 6, 4, 7], 6), true);
    assert_eq!(Solution::check_subarray_sum(vec![23, 2, 6, 4, 7], 13), false);

    assert_eq!(Solution::check_subarray_sum(vec![1, 1, 6, 1, 1], 6), false);
    assert_eq!(Solution::check_subarray_sum(vec![1, 1, 6, 6, 1, 1], 6), true);
    assert_eq!(Solution::check_subarray_sum(vec![1, 1, 0, 1, 1], 6), false);
    assert_eq!(Solution::check_subarray_sum(vec![1, 1, 0, 0, 1, 1], 6), true);

    let mut vec = Vec::new();
    vec.resize(10000, 2);
    assert_eq!(Solution::check_subarray_sum(vec.clone(), 2), true);
    assert_eq!(Solution::check_subarray_sum(vec.clone(), 20001), false);

    let mut vec = Vec::new();
    vec.resize(10000, 1);
    vec[9998] = 10001;
    vec[9999] = 10001;
    assert_eq!(Solution::check_subarray_sum(vec.clone(), 10001), true);
}

use std::time::Instant;
struct Solution;

fn main() {
    run_test("test case 0", test_case_0);
}

fn run_test(name: &str, test_fn: fn() -> ()) {
    let time = Instant::now();
    test_fn();
    let time = Instant::now() - time;
    println!("'{name}' success in {}ms", time.as_secs_f32() * 1000.0);
}
