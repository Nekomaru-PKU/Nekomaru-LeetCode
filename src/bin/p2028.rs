fn solution(rolls: Vec<i32>, mean: i32, n: i32) -> Vec<i32> {
    let m = rolls.len() as i32;
    let sum = rolls.iter().copied().sum::<i32>();
    let sum_expected = mean * (m + n);
    let sum_missing = sum_expected - sum;
    (sum_missing >= n && sum_missing <= n * 6)
        .then(|| {
            let value = sum_missing / n;
            let count = ((value + 1) * n - sum_missing) as usize;
            let mut out = vec![0; n as _];
            out[..count].fill(value);
            out[count..].fill(value + 1);
            out
        })
        .unwrap_or_default()
}

fn main() {
    assert_eq!(solution(vec![3, 2, 4, 3], 4, 2), [6, 6]);
    assert_eq!(solution(vec![1, 5, 6], 3, 4), [2, 2, 2, 3]);
    assert_eq!(solution(vec![1, 2, 3, 4], 6, 4), []);
}
