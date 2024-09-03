fn solution(s: String) -> String {
    let s = s.into_bytes();
    let n = s.len();
    String::from_utf8((0..n)
        .flat_map(|mid| Iterator::chain(
            // check for s[(mid - len)..mid] == s[mid..(mid + len)].rev()
            // => check for s[mid - i] == s[mid + i - 1]
            // => requiring mid - i >= 0 && mid + i - 1 < n
            // => requiring i <= mid && i < n - mid + 1
            // => requiring i <= mid.min(n - mid)
            (1..=mid.min(n - mid))
                .take_while(|&i| s[mid - i] == s[mid + i - 1])
                .last()
                .map(|len| &s[(mid - len)..(mid + len)])
                .into_iter()
        ,
            // check for s[(mid - len)..mid] == s[(mid + 1)..(mid + len + 1)].rev()
            // => check for s[mid - i] == s[mid + i]
            // => requiring mid - i>= 0 && mid + i < n
            // => requiring i <= mid && i < n - mid
            // => requiring i <= mid.min(n - mid - 1)
            (0..=mid.min(n - mid - 1))
                .take_while(|&i| s[mid - i] == s[mid + i])
                .last()
                .map(|len| &s[(mid - len)..=(mid + len)])
        ))
        .max_by_key(|&s| s.len())
        .unwrap()
        .to_vec()).unwrap()
}

fn main() {
    assert_eq!(solution("babad".into()), "aba");
    assert_eq!(solution("cbbd".into()), "bb");
    assert_eq!(solution("abc".into()), "c");
    assert_eq!(solution("aacabdkacaa".into()), "aca");
}
