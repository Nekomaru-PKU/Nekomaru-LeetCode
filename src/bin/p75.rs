#![expect(clippy::ptr_arg)]

fn solution_two_pass(nums: &mut Vec<i32>) {
    let mut count = [0; 3];
    for &num in &*nums {
        count[num as usize] += 1;
    }

    let end_0 = count[0] as usize;
    let end_1 = end_0 + count[1] as usize;
    let end_2 = end_1 + count[2] as usize;
    nums[..end_0].fill(0);
    nums[end_0..end_1].fill(1);
    nums[end_1..end_2].fill(2);
}

fn solution_one_pass(nums: &mut Vec<i32>) {
    // we maintain the following invariant that:
    // - `end_0`, `end_1` and `end_2` are in range `0..=nums.len()` and
    //    satisfies `end_0 <= end_1 <= end_2`;
    // - `nums[     ..end_0]` contains only `0`s;
    // - `nums[end_0..end_1]` contains only `1`s;
    // - `nums[end_1..end_2]` contains only `2`s;
    // - `nums[end_2..     ]` has not been checked yet.
    // and we keep checking `nums[end_2]` until `end_2 == nums.len()`.
    let mut end_0 = 0;
    let mut end_1 = 0;
    let mut end_2 = 0;
    while end_2 < nums.len() {
        match nums[end_2] {
            0 => {
                nums.swap(end_2, end_0);
                end_0 += 1;
                end_1 = end_1.max(end_0);
                end_2 = end_2.max(end_1);
            },
            1 => {
                nums.swap(end_2, end_1);
                end_1 += 1;
                end_2 = end_2.max(end_1);
            },
            2 => {
                end_2 += 1;
            },
            _ => unreachable!("invalid input"),
        }
    }
}

fn main() {
    fn apply_fn<T>(func: fn(&mut T), mut data: T) -> T {
        func(&mut data);
        data
    }

    for solution in [
        solution_one_pass,
        solution_two_pass,
    ] {
        // cases from leet code:
        assert_eq!(apply_fn(solution, vec![2, 0, 2, 1, 1, 0]), &[0, 0, 1, 1, 2, 2]);
        assert_eq!(apply_fn(solution, vec![2, 0, 1]), &[0, 1, 2]);

        // cases of simple edge cases:
        assert_eq!(apply_fn(solution, vec![0]), &[0]);
        assert_eq!(apply_fn(solution, vec![1]), &[1]);
        assert_eq!(apply_fn(solution, vec![2]), &[2]);
        assert_eq!(apply_fn(solution, vec![0, 1]), &[0, 1]);
        assert_eq!(apply_fn(solution, vec![0, 2]), &[0, 2]);
        assert_eq!(apply_fn(solution, vec![1, 2]), &[1, 2]);
        assert_eq!(apply_fn(solution, vec![0, 1, 2]), &[0, 1, 2]);
        assert_eq!(apply_fn(solution, vec![0, 2, 1]), &[0, 1, 2]);
        assert_eq!(apply_fn(solution, vec![1, 0, 2]), &[0, 1, 2]);
        assert_eq!(apply_fn(solution, vec![1, 2, 0]), &[0, 1, 2]);
        assert_eq!(apply_fn(solution, vec![2, 0, 1]), &[0, 1, 2]);
        assert_eq!(apply_fn(solution, vec![2, 1, 0]), &[0, 1, 2]);
    }
}
