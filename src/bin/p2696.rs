fn solution(s: String) -> i32 {
    let s = s.as_bytes();
    let mut stack = Vec::with_capacity(s.len());
    for &c in s {
        if matches!(
            (stack.last().copied(), c),
            (Some(b'A'), b'B') |
            (Some(b'C'), b'D')) {
            stack.pop();
        } else {
            stack.push(c);
        }
    }
    stack.len() as _
}

fn main() {
    assert_eq!(solution("ABFCACDB".into()), 2);
    assert_eq!(solution("ACBBD".into()), 5);
}
