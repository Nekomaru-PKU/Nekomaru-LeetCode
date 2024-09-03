pub fn solution(height: Vec<i32>) -> i32 {
    use std::cmp::Ordering;
    let mut head = 0;
    let mut tail = height.len() - 1;
    let mut max_area = 0;
    while head < tail {
        max_area = max_area.max(
            i32::min(height[head], height[tail]) *
            (tail - head) as i32);
        match i32::cmp(&height[head], &height[tail]) {
            Ordering::Less    => head += 1,
            Ordering::Greater => tail -= 1,
            Ordering::Equal   => {
                head += 1;
                tail -= 1;
            }
        }
    }
    max_area
}

fn main() {
    assert_eq!(solution(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    assert_eq!(solution(vec![1, 1]), 1);
}
