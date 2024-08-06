fn solution(s: String) -> i32 {
    let s = s.into_bytes();
    let mut s = &s[..];
    while let Some(b' ') = s.first() {
        s = &s[1..];
    }

    let (s, pos) = match s.first() {
        Some(b'0'..=b'9') => (s, true),
        Some(b'+') => (&s[1..], true),
        Some(b'-') => (&s[1..], false),
        _ => return 0,
    };

    let mut x = 0i32;
    for c in s {
        let digit = if c.is_ascii_digit() {
            c - b'0'
        } else {
            break
        } as i32;
        x = x.saturating_mul(10);
        x = if pos {
            x.saturating_add(digit)
        } else {
            x.saturating_sub(digit)
        };
    }
    x
}

fn main() {
    assert_eq!(solution("42".into()), 42);
    assert_eq!(solution("   -042".into()), -42);
    assert_eq!(solution("1337c0d3".into()), 1337);
    assert_eq!(solution("0-1".into()), 0);
    assert_eq!(solution("words and 987".into()), 0);

    assert_eq!(solution(format!("  {}+asdf", i32::MAX)), i32::MAX);
    assert_eq!(solution(format!(" -{}+asdf", i32::MAX)), -i32::MAX);
    assert_eq!(solution(format!("  {}+asdf", i32::MIN)), i32::MIN);
    assert_eq!(solution(format!(" -{}+asdf", i32::MIN)), 0);
}
