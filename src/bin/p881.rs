mod solution {
    use super::*;

    pub fn main(mut people: Vec<i32>, limit: i32) -> i32 {
        people.sort_unstable();

        let mut taken = Vec::new();
        taken.resize(people.len(), false);

        let mut boats = 0;
        for i in (0..people.len()).rev() {
            if taken[i] {
                continue;
            }

            // here `people[i]` is the heaviest person that we haven't put into
            // a boat yet.
            // let's find the haviest `people[j]` that can fit into a boat with
            // `people[i]`.
            let j = binary_search::find_last_element_less_equal_than(
                &people[0..i],
                limit - people[i]);
            if let Some(j) = j {
                for j_not_taken in (0..=j).rev() {
                    if !taken[j_not_taken] {
                        taken[j_not_taken] = true;
                        break;
                    }
                }
            }
            boats += 1;
        }

        boats
    }

}

mod binary_search {
    pub fn find_last_element_less_equal_than(nums: &[i32], target: i32) -> Option<usize> {
        if nums.is_empty() || nums[0] > target {
            return None;
        }

        let mut begin = 0;
        let mut end = nums.len();
        while end - begin > 1 {
            let mid = begin + (end - begin) / 2;
            if nums[mid] > target {
                end = mid;
            } else {
                begin = mid;
            }
        }
        Some(begin)
    }

    #[test]
    fn test_find_last_element_less_equal_than() {
        assert_eq!(find_last_element_less_equal_than(&[1, 2, 3, 4, 5], 0), None);
        assert_eq!(find_last_element_less_equal_than(&[1, 2, 3, 4, 5], 1), Some(0));
        assert_eq!(find_last_element_less_equal_than(&[1, 2, 3, 4, 5], 2), Some(1));
        assert_eq!(find_last_element_less_equal_than(&[1, 2, 3, 4, 5], 3), Some(2));
        assert_eq!(find_last_element_less_equal_than(&[1, 2, 3, 4, 5], 4), Some(3));
        assert_eq!(find_last_element_less_equal_than(&[1, 2, 3, 4, 5], 5), Some(4));
        assert_eq!(find_last_element_less_equal_than(&[1, 2, 3, 4, 5], 6), Some(4));
    }
}

use nekomaru_leetcode::print_time;

fn main() {
    assert_eq!(solution::main(vec![1, 2], 3), 1);
    assert_eq!(solution::main(vec![3, 2, 2, 1], 3), 3);
    assert_eq!(solution::main(vec![3, 5, 3, 4], 5), 4);

    assert_eq!(solution::main(vec![1, 2, 3, 4], 5), 2);

    assert_eq!(solution::main(vec![5, 1, 7, 4, 2, 4], 7), 4);

    let people = (1..=50000).map(|_| 50000).collect::<Vec<_>>();
    let result = print_time("perf", || solution::main(people, 50000));
    // in this case everyone is on its own boat.
    assert_eq!(result, 50000);

    let people = (1..50000).collect::<Vec<_>>();
    let result = print_time("perf", || solution::main(people, 50000));
    // in this case we pair `people[i]` with `people[50000 - i]` for each `i`
    // from 0 to 24999 and we have `people[25000]` left, so we need 25000 boats.
    assert_eq!(result, 25000);

    let people = (1..=50000).collect::<Vec<_>>();
    let result = print_time("perf", || solution::main(people, 50001));
    // in this case we pair `people[i]` with `people[50001 - i]` for each `i`
    // from 0 to 25000, so we need 25000 boats.
    assert_eq!(result, 25000);
}
