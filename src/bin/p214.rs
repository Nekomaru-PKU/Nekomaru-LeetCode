pub fn solution(s: String) -> String {
    if s.is_empty() { return s; }
    fn is_palindrome(s: &[u8]) -> bool {
        (0..s.len() / 2).all(|i| {
            s[i] ==
            s[s.len() - i - 1]
        })
    }
    let s = s.into_bytes();
    let len =
        (1..=s.len())
            .rev()
            .find(|&len| is_palindrome(&s[..len]))
            .unwrap();
    let mut out = s[len..].to_owned();
    out.reverse();
    out.extend_from_slice(&s);
    String::from_utf8(out).unwrap()
}

fn main() {
    assert_eq!(solution("aacecaaa".into()), "aaacecaaa");
    assert_eq!(solution("abcd".into()), "dcbabcd");
}
