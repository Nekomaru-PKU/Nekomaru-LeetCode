mod solution {
    pub fn main(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let nums = {
            let mut freq = [0u32; 64];
            for &num in &nums {
                freq[num as usize] += 1;
            }
            freq.iter()
                .enumerate()
                .map(|(num, &freq)| (num as _, freq))
                .collect::<Vec<_>>()
        };

        let mut results = Vec::new();
        let mut stack = Vec::new();
        search(&nums, target, &mut stack, &mut results);
        results
    }

    fn search(
        nums: &[(i32, u32)],
        mut target: i32,
        stack: &mut Vec<i32>,
        results: &mut Vec<Vec<i32>>) {
        if let Some(&(num, mut freq)) = nums.first() {
            let len = stack.len();

            search(&nums[1..], target, stack, results);
            while target >= num && freq >= 1 {
                stack.push(num);
                freq -= 1;
                target -= num;
                if target > 0 {
                    search(&nums[1..], target, stack, results);
                } else {
                    results.push(stack.clone());
                }
            }

            stack.truncate(len);
        }
    }
}

fn main() {
    assert!(leetcode::cmp::eq_any_order(
        &solution::main(vec![10, 1, 2, 7, 6, 1, 5], 8),
        &[
            vec![1, 1, 6],
            vec![1, 2, 5],
            vec![1, 7],
            vec![2, 6],
        ]));
    assert!(leetcode::cmp::eq_any_order(
        &solution::main(vec![2, 5, 2, 1, 2], 5),
        &[
            vec![1, 2, 2],
            vec![5],
        ]));
    assert!(leetcode::cmp::eq_any_order(
        &solution::main(vec![1; 100], 30),
        &[vec![1; 30]]));
}
