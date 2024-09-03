fn solution(nums: Vec<i32>, target: i32) -> Vec<i32> {
    // <time: O(n), space: O(n)>

    // let map[num] := nums.last_index_of(num)
    // note that when collecting tuples into a `HashMap`, any
    // tuples with the same keys will be overwritten by the
    // last one encountered without error or panic, which is
    // fine for this case.
    use std::collections::HashMap;
    let map = nums
        .iter()
        .enumerate()
        .map(|(idx, &num)| (num, idx as i32))
        .collect::<HashMap<_, _>>();

    for (i, &a) in nums.iter().enumerate() {
        let i = i as i32;
        let b = target - a;

        // if a == b, we are not sure that whether that number
        // occurs more than once in `nums` by checking `map`.
        // as such check takes O(n), we do such check later.
        if a == b { continue; }

        if let Some(&j) = map.get(&b) {
            return vec![i, j];
        }
    }

    // as the problem description shows, each input would have
    // exactly one solution. since we have not found any solution
    // yet, we are sure that the solution is about `target / 2`
    // that occurs in `nums` exactly twice and just search for them.
    let half = target / 2;
    nums.iter()
        .enumerate()
        .filter(|&(_, &num)| num == half)
        .take(2)
        .map(|(idx, _)| idx as _)
        .collect()
}

fn main() {
    assert_eq!(solution(vec![2, 7, 11, 15], 9), [0, 1]);
    assert_eq!(solution(vec![3, 2, 4], 6), [1, 2]);
    assert_eq!(solution(vec![3, 3], 6), [0, 1]);
}
