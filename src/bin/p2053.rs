fn solution(arr: Vec<String>, k: i32) -> String {
    enum Freq { Once(usize), MoreThanOnce }

    use std::collections::HashMap;
    let mut freq = HashMap::new();
    for (idx, str) in arr.into_iter().enumerate() {
        freq.entry(str)
            .and_modify(|freq| *freq = Freq::MoreThanOnce)
            .or_insert(Freq::Once(idx));
    }

    let mut vec = freq
        .into_iter()
        .filter_map(|(str, freq)|
            if let Freq::Once(idx) = freq {
                Some((str, idx))
            } else {
                None
            })
        .collect::<Vec<_>>();
    vec .sort_unstable_by_key(|&(_, idx)| idx);
    #[cfg(debug_assertions)] println!("{vec:?}");
    vec .into_iter()
        .map(|(str, _)| str)
        .nth((k - 1) as _)
        .unwrap_or_default()
}

fn main() {
    use leetcode::input::Input;
    assert_eq!(solution(["d", "b", "c", "b", "c", "a"].input(), 2), "a");
    assert_eq!(solution(["aaa", "aa", "a"].input(), 1), "aaa");
    assert_eq!(solution(["a", "b", "a"].input(), 3), "");
}
