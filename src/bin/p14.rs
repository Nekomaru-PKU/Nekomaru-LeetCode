fn solution(strs: Vec<String>) -> String {
    let strs = strs
        .into_iter()
        .map(String::into_bytes)
        .collect::<Vec<_>>();
    let lcp_len = (0..strs[0].len())
        .find(|&i| {
            strs.iter()
                .any(|s| i >= s.len() || s[i] != strs[0][i])
        })
        .unwrap_or(strs[0].len());
    String::from_utf8(strs[0][..lcp_len].to_vec()).unwrap()
}

fn main() {
    use leetcode::input::Input;
    assert_eq!(solution(["flower", "flow", "flight"].input()), "fl");
    assert_eq!(solution(["dog", "racecar", "car"].input()), "");
}
