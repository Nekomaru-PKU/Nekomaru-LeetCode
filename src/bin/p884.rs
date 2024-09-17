
fn solution(s1: String, s2: String) -> Vec<String> {
    use core::ops::AddAssign;
    use std::collections::HashMap;
    let mut freq = HashMap::<&str, u32>::new();
    for word in Iterator::chain(
        s1.split(' '),
        s2.split(' ')) {
        freq.entry(word)
            .or_default()
            .add_assign(1);
    }
    freq.iter()
        .filter(|&(_, &freq)| freq == 1)
        .map(|(&word, _)| word.to_owned())
        .collect()
}

fn main() {
    use leetcode::cmp;
    assert!(cmp::eq_any_order(
        solution(
            "this apple is sweet".into(),
            "this apple is sour".into()),
        ["sweet", "sour"]));
    assert_eq!(
        solution(
            "apple apple".into(),
            "banana".into()),
        ["banana"]);
}
