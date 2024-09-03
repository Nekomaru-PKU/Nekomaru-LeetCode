fn solution(mapping: Vec<i32>, nums: Vec<i32>) -> Vec<i32> {
    const fn map_digits(mapping: &[i32], num: i32) -> i32 {
        if num != 0 {
            let mut num = num;
            let mut out = 0;
            let mut exp = 1;
            while num != 0 {
                out += mapping[(num % 10) as usize] * exp;
                num /= 10;
                exp *= 10;
            }
            out
        } else {
            mapping[0]
        }
    }

    let mut pairs = nums.iter()
        .map(|&num| (num, map_digits(&mapping, num)))
        .collect::<Vec<_>>();
    pairs.sort_by_key(|&(_, mapped)| mapped);
    pairs.iter()
        .map(|&(num, _)| num)
        .collect()
}

fn main() {
    assert_eq!(solution(
        vec![8, 9, 4, 0, 2, 1, 3, 5, 7, 6],
        vec![991, 338, 38]),
        vec![338, 38, 991]);
    assert_eq!(solution(
        vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
        vec![789, 456, 123]),
        vec![123, 456, 789]);
    assert_eq!(solution(
        vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0],
        vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]),
        vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
}
