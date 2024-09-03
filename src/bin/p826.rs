mod binary_search {
    pub fn find_last_by_key_less_equal_than<T, K: Ord>(
        nums: &[T],
        key_fn: fn(&T) -> K,
        target: K,
    ) -> Option<usize> {
        if nums.is_empty() || key_fn(&nums[0]) > target {
            return None;
        }

        let mut begin = 0;
        let mut end = nums.len();
        while end - begin > 1 {
            let mid = begin + (end - begin) / 2;
            if key_fn(&nums[mid]) > target {
                end = mid;
            } else {
                begin = mid;
            }
        }
        Some(begin)
    }

    #[test]
    fn test_find_last_element_less_equal_than() {
        assert_eq!(find_last_by_key_less_equal_than(&[1, 2, 3, 4, 5], |&o| o, 0), None);
        assert_eq!(find_last_by_key_less_equal_than(&[1, 2, 3, 4, 5], |&o| o, 1), Some(0));
        assert_eq!(find_last_by_key_less_equal_than(&[1, 2, 3, 4, 5], |&o| o, 2), Some(1));
        assert_eq!(find_last_by_key_less_equal_than(&[1, 2, 3, 4, 5], |&o| o, 3), Some(2));
        assert_eq!(find_last_by_key_less_equal_than(&[1, 2, 3, 4, 5], |&o| o, 4), Some(3));
        assert_eq!(find_last_by_key_less_equal_than(&[1, 2, 3, 4, 5], |&o| o, 5), Some(4));
        assert_eq!(find_last_by_key_less_equal_than(&[1, 2, 3, 4, 5], |&o| o, 6), Some(4));
    }
}

fn solution(
    difficulty: Vec<i32>,
    profit: Vec<i32>,
    worker: Vec<i32>,
) -> i32 {
    let jobs = {
        use std::collections::BTreeMap;
        let mut map = BTreeMap::new();
        for (profit, difficulty) in profit.into_iter().zip(difficulty) {
            let best_profit: &mut i32 = map.entry(difficulty).or_default();
            *best_profit = (*best_profit).max(profit);
        }
        let mut vec = map.into_iter().collect::<Vec<(_, i32)>>();
        for i in 1..vec.len() {
            vec[i].1 = vec[i].1.max(vec[i - 1].1);
        }
        vec
    };
    worker.into_iter()
        .map(|ability|
            binary_search::find_last_by_key_less_equal_than(
                &jobs,
                |&(difficulty, _)| difficulty,
                ability)
            .map(|job_index| jobs[job_index].1)
            .unwrap_or_default())
        .sum()
}

fn main() {
    assert_eq!(solution(
        vec![2, 4, 6, 8, 10],
        vec![10, 20, 30, 40, 50],
        vec![4, 5, 6, 7]), 100);
    assert_eq!(solution(
        vec![85, 47, 57],
        vec![24, 66, 99],
        vec![40, 25, 25]), 0);

    assert_eq!(solution(
        vec![68, 35, 52, 47, 86],
        vec![67, 17, 1, 81, 3],
        vec![92, 10, 85, 84, 82]), 324);
}
