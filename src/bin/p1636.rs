fn solution(nums: Vec<i32>) -> Vec<i32> {
    use std::iter;
    let mut freq = [0; 201];
    for num in nums {
        freq[num as usize + 100] += 1;
    }
    let mut freq = (-100..=100)
        .zip(freq)
        .collect::<Vec<_>>();
    freq.sort_unstable_by_key(|&(num, freq)| (freq, -num));
    freq.iter()
        .flat_map(|&(num, freq)| iter::repeat(num).take(freq))
        .collect()
}

fn main() {
    assert_eq!(
        solution(vec![1, 1, 2, 2, 2, 3]),
        [3, 1, 1, 2, 2, 2]);
    assert_eq!(
        solution(vec![2, 3, 1, 3, 2]),
        [1, 3, 3, 2, 2]);
    assert_eq!(
        solution(vec![-1,  1, -6, 4,  5, -6, 1, 4, 1]),
        [ 5, -1,  4, 4, -6, -6, 1, 1, 1]);
}
