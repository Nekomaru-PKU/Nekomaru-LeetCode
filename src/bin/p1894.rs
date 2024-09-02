fn solution(chalk: Vec<i32>, k: i32) -> i32 {
    let sum = chalk
        .iter()
        .scan(0u64, |acc, &num| {
            *acc += num as u64;
            Some(*acc)
        })
        .collect::<Vec<_>>();
    let n = chalk.len();
    let k = k as u64 % sum[n - 1];
    match sum.binary_search(&k) {
        Ok(i) => ((i + 1) % n) as _,
        Err(i) => i as _,
    }
}

fn main() {
    assert_eq!(solution(vec![5, 1, 5], 22), 0);
    assert_eq!(solution(vec![5, 1, 5], 23), 0);
    assert_eq!(solution(vec![3, 4, 1, 2], 25), 1);
}
