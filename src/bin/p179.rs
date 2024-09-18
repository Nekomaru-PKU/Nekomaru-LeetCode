fn solution(nums: Vec<i32>) -> String {
    let mut nums =
        nums.iter()
            .map(i32::to_string)
            .collect::<Vec<_>>();
    nums.sort_unstable_by(|a, b| String::cmp(
        &(a.clone() + b),
        &(b.clone() + a)));
    nums.reverse();
    let out = nums.join("");
    let out = out.trim_start_matches("0");
    out .is_empty()
        .then_some("0".to_owned())
        .unwrap_or(out.to_owned())
}

fn main() {
    assert_eq!(solution(vec![0, 0]), "0");
    assert_eq!(solution(vec![10, 2]), "210");
    assert_eq!(solution(vec![3, 30, 34, 5, 9]), "9534330");
}
