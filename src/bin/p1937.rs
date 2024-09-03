#![expect(clippy::needless_range_loop)]

fn solution(points: Vec<Vec<i32>>) -> i64 {
    let num_rows = points.len();
    let num_cols = points[0].len();

    let mut dp = points[0]
        .iter()
        .map(|&point| point as i64)
        .collect::<Vec<_>>();
    let mut dp_next = vec![0; num_cols];
    for i in 1..num_rows {
        for j in 0..num_cols {
            dp_next[j] =
                points[i][j] as i64 +
                (0..num_cols)
                    .map(|k| dp[k] - (k as i64 - j as i64).abs())
                    .max()
                    .unwrap();
        }
        (dp, dp_next) = (dp_next, dp);
    }

    dp  .iter()
        .copied()
        .max()
        .unwrap()
}

fn main() {
    use leetcode::input::Input;
    assert_eq!(solution([[1, 2, 3], [1, 5, 1], [3, 1, 1]].input()), 9);
    assert_eq!(solution([[1, 5], [2, 3], [4, 2]].input()), 11);
    assert_eq!(solution([[1]].input()), 1);
}
