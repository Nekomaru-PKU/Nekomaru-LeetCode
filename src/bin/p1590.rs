fn solution(nums: Vec<i32>, p: i32) -> i32 {
    // <scale: 1e+5, time: O(n^2), space: O(n)>

    let n = nums.len();
    let p = p as u64;
    let sum = {
        let mut sum = Vec::<u64>::with_capacity(n + 1);
        let mut acc = 0;
        sum.push(0);
        for &num in &nums {
            acc += num as u64;
            sum.push(acc % p);
        }
        sum
    };

    if sum[n] == 0 { return 0; }
    for subarr_len in 1..n {
        for subarr_begin in 0..=(n - subarr_len) {
            let sum_before = sum[subarr_begin];
            let sum_after = sum[subarr_begin + subarr_len];
            let subarr_sum = p + sum_after - sum_before;
            if  subarr_sum == sum[n] ||
                subarr_sum == sum[n] + p {
                return subarr_len as _;
            }
        }
    }
    -1
}

fn main() {
    assert_eq!(solution(vec![3, 1, 4, 2], 6), 1);
    assert_eq!(solution(vec![6, 3, 5, 2], 9), 2);
    assert_eq!(solution(vec![1, 2, 3], 3), 0);
    assert_eq!(solution(vec![3, 3, 3], 7), -1);
}
