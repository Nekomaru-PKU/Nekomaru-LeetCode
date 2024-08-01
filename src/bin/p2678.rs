fn solution(details: Vec<String>) -> i32 {
    details
        .into_iter()
        .map(|s| s.into_bytes())
        .filter(|s| 10 * (s[11] - b'0') + (s[12] - b'0') > 60)
        .count() as _
}

fn main() { let _ = solution(vec![]); }
