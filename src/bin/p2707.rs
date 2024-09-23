fn solution(s: String, dict: Vec<String>) -> i32 {
    use std::collections::HashSet;
    let dict =
        dict.into_iter()
            .map(String::into_bytes)
            .collect::<HashSet<_>>();
    let s = s.into_bytes();
    let n = s.len();
    // dp[i] is the solution for `s[0..i]`.
    let mut dp = vec![0; n + 1];
    for i in 1..=n {
        dp[i] = Ord::min(dp[i - 1] + 1, {
            (1..=i)
                .filter(|&j| dict.contains(&s[(i - j)..i]))
                .map(|j| dp[i - j])
                .min()
                .unwrap_or(i32::MAX)
        });
    }
    dp[n]
}

fn main() {
    use leetcode::input::Input;
    assert_eq!(solution(
        "leetscode".into(),
        ["leet", "code", "leetcode"].input()), 1);
    assert_eq!(solution(
        "sayhelloworld".into(),
        ["hello", "world"].input()), 3);
}
