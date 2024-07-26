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
    fn vec_of_string(vec: &[&'static str]) -> Vec<String> {
        vec.iter().cloned().map(Into::into).collect()
    }

    assert_eq!(solution(
        vec_of_string(&["Mary","John","Emma"]),
        vec![180, 165, 170]),
        vec_of_string(&["Mary","Emma","John"]));
    assert_eq!(solution(
        vec_of_string(&["Alice","Bob","Bob"]),
        vec![155,185,150]),
        vec_of_string(&["Bob","Alice","Bob"]));
}
