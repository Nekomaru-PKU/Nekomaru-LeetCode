fn solution(mut nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let m = nums.iter().filter(|&&num| num == 1).count();

    nums.extend(nums.clone());

    let mut num_swaps = nums[..m]
        .iter()
        .filter(|&&num| num != 1)
        .count() as i32;
    let mut min_swaps = num_swaps;
    for begin in 1..(2 * n - m) {
        if nums[begin - 1] != 1 {
            num_swaps -= 1;
        }
        if nums[begin + m - 1] != 1 {
            num_swaps += 1;
        }
        min_swaps = min_swaps.min(num_swaps);
    }
    min_swaps
}

fn main() {
    assert_eq!(solution(vec![0, 1, 0, 1, 1, 0, 0]), 1);
    assert_eq!(solution(vec![0, 1, 1, 1, 0, 0, 1, 1, 0]), 2);
    assert_eq!(solution(vec![1, 1, 0, 0, 1]), 0);
}
