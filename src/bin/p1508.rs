fn solution(nums: Vec<i32>, _: i32, left: i32, right: i32) -> i32 {
    let n = nums.len();

    let mut sums = Vec::with_capacity(n * (n + 1) / 2);
    for begin in 0..n {
        let mut sum = 0;
        for len in 1..=(n - begin) {
            sum += nums[begin + len - 1];
            sums.push(sum);
        }
    }

    sums.sort_unstable();

    let mut total_sum = 0;
    for &sum in &sums[((left - 1) as _)..(right as _)] {
        total_sum += sum as i64;
        total_sum %= 1_000_000_007;
    }

    total_sum as _
}

fn main() {
    assert_eq!(solution(vec![1, 2, 3, 4], 4, 1, 5), 13);
    assert_eq!(solution(vec![1, 2, 3, 4], 4, 3, 4), 6);
    assert_eq!(solution(vec![1, 2, 3, 4], 4, 1, 10), 50);
}
