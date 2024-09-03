fn solution(nums: Vec<i32>) -> Vec<Vec<i32>> {
    use std::collections::HashMap;
    let mut freq = HashMap::<_, u32>::with_capacity(nums.len());
    for &num in &nums {
        *freq.entry(num).or_default() += 1;
    }

    let mut nums = freq
        .keys()
        .cloned()
        .collect::<Vec<_>>();
    nums.sort_unstable();

    // we assume that in each triplet, `nums[i] <= nums[j] <= nums[k]`.
    let mut out = Vec::new();
    for (i, &nums_i) in nums
        .iter()
        .enumerate()
        .take_while(|&(_, &num)| num <= 0) {
        let freq_i = freq[&nums_i];
        // as we require that `nums[i] + nums[j] + nums[k] = 0`,
        // we can assert that `nums[i] + 2 * nums[j] <= 0`.
        for &nums_j in nums[i..]
            .iter()
            .take_while(|&num_j| nums_i + 2 * num_j <= 0) {
            let freq_j = freq[&nums_j];
            let nums_k = -(nums_i + nums_j);

            #[allow(clippy::nonminimal_bool)]
            if  !(nums_i == nums_j && nums_j == nums_k && freq_j < 3) &&
                !(nums_i == nums_j && freq_j < 2) &&
                !(nums_i == nums_k && freq_i < 2) &&
                !(nums_j == nums_k && freq_j < 2) &&
                freq.contains_key(&nums_k) {
                out.push(vec![nums_i, nums_j, nums_k]);
            }
        }
    }
    out
}

fn main() {
    assert_eq!(solution(vec![-1, 0, 1, 2, -1, -4]), [[-1, -1, 2], [-1, 0, 1]]);
    assert_eq!(solution(vec![1, 0, -1, -2, 1, 4]), [[-2, 1, 1], [-1, 0, 1]]);
    assert_eq!(solution(vec![0, 1, 1]), [[]; 0]);
    assert_eq!(solution(vec![0, 0, 0]), [[0, 0, 0]]);
}
