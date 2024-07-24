mod solution {
    use std::iter;

    pub fn main(nums: Vec<i32>) -> Vec<i32> {
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
}

fn main() {
    assert_eq!(solution::main(
        vec![1, 1, 2, 2, 2, 3]),
        vec![3, 1, 1, 2, 2, 2]);
    assert_eq!(solution::main(
        vec![2, 3, 1, 3, 2]),
        vec![1, 3, 3, 2, 2]);
    assert_eq!(solution::main(
        vec![-1,  1, -6, 4,  5, -6, 1, 4, 1]),
        vec![ 5, -1,  4, 4, -6, -6, 1, 1, 1]);
}
