fn solution(mut nums: Vec<i32>, k: i32) -> i32 {
    nums.sort_unstable();

    use std::collections::BinaryHeap;
    let mut heap = BinaryHeap::from(vec![i32::MAX; k as _]);

    let mut begins = (0..nums.len()).collect::<Vec<_>>();
    let mut begins_next = Vec::with_capacity(nums.len());
    let mut len = 1;
    while !begins.is_empty() {
        begins_next.clear();
        for &begin in &begins {
            if begin + len < nums.len() {
                let diff = nums[begin + len] - nums[begin];
                if  diff < heap.peek().copied().unwrap() {
                    heap.pop();
                    heap.push(diff);
                    begins_next.push(begin);
                }
            }
        }

        (begins, begins_next) = (begins_next, begins);
        len += 1;
    }

    heap.peek().copied().unwrap()
}

fn solution_opt(mut nums: Vec<i32>, k: i32) -> i32 {
    nums.sort_unstable();

    let n = nums.len();
    let k = k as usize;

    let mut min = (0..n - 1)
        .map(|i| nums[i + 1] - nums[i])
        .min()
        .unwrap();
    let mut max = nums[n - 1] - nums[0];

    while min < max {
        let mid = (min + max + 1) / 2;

        let mut m = 0;
        let mut begin = 0;
        let mut len = 0;
        while begin < n {
            while begin + len + 1 < n &&
                nums[begin + len + 1] - nums[begin] < mid {
                m += 1;
                len += 1;
            }

            begin += 1;
            len -= 1;
            m += len;
        }

        println!("seaching at {mid} in [{min}, {max}], k = {k}, m = {m}");

        if m >= k - 1 {
            max = mid - 1;
        } else {
            min = mid;
        }
    }

    min
}

fn main() {
    assert_eq!(crate::solution(vec![1, 3, 1], 1), 0);
    assert_eq!(crate::solution(vec![1, 1, 1], 2), 0);
    assert_eq!(crate::solution(vec![1, 6, 1], 3), 5);

    assert_eq!(crate::solution_opt(vec![1, 3, 1], 1), 0);
    assert_eq!(crate::solution_opt(vec![1, 1, 1], 2), 0);
    assert_eq!(crate::solution_opt(vec![1, 6, 1], 3), 5);

    assert_eq!(crate::solution_opt(vec![62, 100, 4], 2), 58);
    assert_eq!(crate::solution_opt(vec![38,33,57,65,13,2,86,75,4,56], 26), 36);

    fn test_perf(n: i32) {
        let _ = crate::solution_opt((0..n as _).collect(), n * (n - 1) / 2);
    }

    leetcode::perf::time("1e3", || test_perf(1_000));
    leetcode::perf::time("3e3", || test_perf(3_000));
    leetcode::perf::time("5e3", || test_perf(5_000));
    leetcode::perf::time("1e4", || test_perf(10_000));
}
