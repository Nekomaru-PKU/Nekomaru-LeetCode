#![allow(clippy::ptr_arg)]

mod solution {
    pub fn main(nums: &mut Vec<i32>) {
        #[allow(path_statements)] {
            self::main_two_pass(nums);
            self::main_one_pass;
        }
    }

    pub fn main_two_pass(nums: &mut Vec<i32>) {
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

    pub fn main_one_pass(nums: &mut Vec<i32>) {
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
                _ => panic!("invalid input"),
            }
        }
    }
}

fn main() {
    fn test_case(input: &[i32], expected: &[i32]) {
        let mut nums = input.to_vec();
        solution::main(&mut nums);
        assert_eq!(nums, expected);
    }

    // cases from leet code:
    test_case(&[2,0,2,1,1,0], &[0,0,1,1,2,2]);
    test_case(&[2,0,1], &[0,1,2]);

    // cases of simple edge cases:
    test_case(&[0], &[0]);
    test_case(&[1], &[1]);
    test_case(&[2], &[2]);
    test_case(&[0, 1], &[0, 1]);
    test_case(&[0, 2], &[0, 2]);
    test_case(&[1, 2], &[1, 2]);
    test_case(&[0, 1, 2], &[0, 1, 2]);
    test_case(&[0, 2, 1], &[0, 1, 2]);
    test_case(&[1, 0, 2], &[0, 1, 2]);
    test_case(&[1, 2, 0], &[0, 1, 2]);
    test_case(&[2, 0, 1], &[0, 1, 2]);
    test_case(&[2, 1, 0], &[0, 1, 2]);
}
