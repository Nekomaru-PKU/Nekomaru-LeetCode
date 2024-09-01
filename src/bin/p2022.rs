fn solution(vec: Vec<i32>, m: i32, n: i32) -> Vec<Vec<i32>> {
    // note that we swapped the meaning of `n` and `m`
    // to match our conventions.
    let (n, m) = (m as usize, n as usize);
    if vec.len() == m * n {
        (0..n).map(|i| vec[i * m..][..m].to_vec()).collect()
    } else {
        vec![]
    }
}

fn main() {
    assert_eq!(solution(vec![1, 2, 3, 4], 2, 2), [[1, 2], [3, 4]]);
    assert_eq!(solution(vec![1, 2, 3], 1, 3), [[1, 2, 3]]);
    assert_eq!(solution(vec![1, 2], 1, 1), [[]; 0]);
}
