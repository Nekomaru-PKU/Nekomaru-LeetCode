mod solution {
    pub fn main(nums: Vec<i32>) -> Vec<i32> {
        use std::iter;

        let mut freq = [0; 201];
        for num in nums {
            freq[num as usize + 100] += 1;
        }

        let mut freq = (-100..=100)
            .zip(freq)
            .collect::<Vec<_>>();
        freq.sort_by_key(|&(num, freq)| (freq, -num));
        freq.into_iter()
            .flat_map(|(num, freq)| iter::repeat(num).take(freq))
            .collect()
    }
}

fn main() {
    assert_eq!(solution::main(
        vec![1, 1, 2, 2, 2, 3]),
        vec![3, 1, 1, 2, 2, 2]);
}
