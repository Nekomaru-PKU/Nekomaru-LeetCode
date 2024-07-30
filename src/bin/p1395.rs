fn solution(rating: Vec<i32>) -> i32 {
    let n = rating.len();

    // we assign each soldier a new rating in range `0..n`
    // while keeping the original ordering.
    let rating = {
        let mut rating_sorted = rating.clone();
        rating_sorted.sort_unstable();

        use std::collections::HashMap;
        let rating_map = rating_sorted.iter()
            .enumerate()
            .map(|(i, &rating)| (rating, i))
            .collect::<HashMap<_, _>>();

        rating.iter()
            .map(|rating| rating_map[rating])
            .collect::<Vec<_>>()
    };

    // `sum[i][j]` is the number of soldiers with index
    // less or equal than `i` and rating less or equal
    // than `j`.
    // `sum` is at most 1024 x 1024 x 8B = 8MiB.
    let mut sum = vec![vec![]; n];
    for i in 0..n {
        sum[i] = if i == 0 {
            vec![0; n]
        } else {
            sum[i - 1].clone()
        };
        for sum in &mut sum[i][rating[i]..n] {
            *sum += 1;
        }
    }

    (1..(n - 1))
        .filter(|&i| rating[i] > 0 && rating[i] < n - 1)
        .map(|i| {
            let j = rating[i];
            let a = sum[i - 1][j - 1];
            let b = sum[i - 1][n - 1] - sum[i - 1][j];
            let c = sum[n - 1][j - 1] - sum[i][j - 1];
            let d = n as i16 - a - b - c - 1;
            (a as i32) * (d as i32) + (b as i32) * (c as i32)
        })
        .sum()
}

fn main() {
    assert_eq!(solution(vec![2, 5, 3, 4, 1]), 3);
    assert_eq!(solution(vec![2, 1, 3]), 0);
    assert_eq!(solution(vec![1, 2, 3, 4]), 4);
}
