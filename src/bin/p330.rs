mod solution {
    pub fn main(mut nums: Vec<i32>, n: i32) -> i32 {
        let mut patches = 0;

        // make sure that `nums.contains(1)`.
        if nums[0] != 1 {
            nums.insert(0, 1);
            patches += 1;
        }

        // here we track `next_missing` as the minimum integer that can not
        // be formed by the sum of a subset of `nums[0..next_num]`.
        // we start with `next_missing = 2` and `next_num = 1` since we have
        // ensured that `num[0] = 1`.
        let mut next_missing: i64 = 2;
        let mut next_num = 1;
        while next_missing <= n as _ {
            // first we try to resolve `next_missing` by considering the next
            // item from `nums`.
            // this is possible if and only if `nums[next_num]` is no greater
            // than `next_missing`.
            if next_num < nums.len() && nums[next_num] <= next_missing as _ {
                // each of `1..next_missing` becomes `(1 + nums[next_num])..`
                // `(next_missing + nums[next_num])`, so now we have resolved
                // `1..(next_missing + nums[next_num])`.
                next_missing += nums[next_num] as i64;
                next_num += 1;
            } else {
                // currently `nums[next_num]` is not helpful. so we add a patch
                // of `next_missing`.
                nums.insert(next_num, next_missing as _);
                patches += 1;
            }
        }

        patches
    }
}

fn main() {
    assert_eq!(solution::main(vec![1, 3], 6), 1);
    assert_eq!(solution::main(vec![1, 5, 10], 20), 2);
    assert_eq!(solution::main(vec![1, 2, 2], 5), 0);

    assert_eq!(solution::main(vec![1,2,31,33], i32::MAX), 28);
}
