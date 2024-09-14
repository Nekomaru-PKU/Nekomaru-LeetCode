fn solution(nums: Vec<i32>) -> i32 {
    use std::cmp::Ordering;
    let mut max = 0;
    let mut best_repeat = 0;
    let mut curr_repeat = 0;
    for &num in &nums {
        match num.cmp(&max) {
            Ordering::Less => curr_repeat = 0,
            Ordering::Equal => {
                curr_repeat += 1;
                best_repeat = best_repeat.max(curr_repeat);
            },
            Ordering::Greater => {
                max = num;
                curr_repeat = 1;
                best_repeat = 1;
            },
        }
    }
    best_repeat
}

fn main() {
    assert_eq!(solution(vec![1, 2, 3, 4]), 1);
    assert_eq!(solution(vec![1, 2, 3, 3, 2, 2]), 2);
    assert_eq!(solution(vec![1, 2, 3, 3, 2, 2, 3, 3, 3]), 3);
}
