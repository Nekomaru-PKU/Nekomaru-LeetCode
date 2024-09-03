fn solution(s: String, k: i32) -> i32 {
    let mut num = s
        .into_bytes()
        .into_iter()
        .map(|c| (c - b'a' + 1) as i32)
        .flat_map(|n| if n >= 10 {
            [Some(n/ 10), Some(n % 10)]
        } else {
            [Some(n), None]
        })
        .flatten()
        .sum::<i32>();
    for _ in 0..(k - 1) {
        let mut acc = 0;
        while num > 0 {
            acc += num % 10;
            num /= 10;
        }
        num = acc;
    }
    num
}

fn main() {
    assert_eq!(solution("iiii".into(), 1), 36);
    assert_eq!(solution("leetcode".into(), 2), 6);
    assert_eq!(solution("zbax".into(), 2), 8);
}
