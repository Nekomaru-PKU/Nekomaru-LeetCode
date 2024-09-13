fn solution(arr: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let acc = arr
        .iter()
        .scan(0, |acc, &num| { *acc ^= num; Some(*acc) })
        .collect::<Vec<_>>();
    queries
        .iter()
        .map(|vec| {
            acc[vec[1] as usize] ^
            (vec[0] > 0)
                .then(|| acc[vec[0] as usize - 1])
                .unwrap_or_default()
        })
        .collect()
}

fn main() {
    use leetcode::input::Input;
    assert_eq!(
        solution(
            vec![1, 3, 4, 8],
            [[0, 1], [1, 2], [0, 3], [3, 3]].input()),
        [2, 7, 14, 8]);
    assert_eq!(
        solution(
            vec![4, 8, 2, 10],
            [[2, 3], [1, 3], [0, 0], [0, 3]].input()),
        [8, 0, 4, 4]);
}
