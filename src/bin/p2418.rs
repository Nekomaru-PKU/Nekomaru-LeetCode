fn solution(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
    let mut zipped = names.into_iter()
        .zip(heights)
        .collect::<Vec<_>>();
    zipped.sort_unstable_by_key(|&(_, height)| -height);
    zipped.into_iter()
        .map(|(name, _)| name)
        .collect()
}

fn main() {
    use leetcode::input::Input;
    assert_eq!(solution(
        ["Mary","John","Emma"].input(),
        [180, 165, 170].input()),
        ["Mary","Emma","John"].input());
    assert_eq!(solution(
        ["Alice","Bob","Bob"].input(),
        [155, 185, 150].input()),
        ["Bob","Alice","Bob"].input());
}
