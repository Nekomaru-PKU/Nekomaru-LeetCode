fn solution(nums: Vec<i32>) -> String {
    let mut nums =
        nums.iter()
            .map(i32::to_string)
            .collect::<Vec<_>>();
    nums.sort_unstable_by(|a, b| Ord::cmp(
        &format!("{a}{b}"),
        &format!("{b}{a}")));
    nums.reverse();
    let out = nums.join("");
    let out = out.trim_start_matches('0');
    if !out.is_empty() {
        out
    } else {
        "0"
    }.to_owned()
}

fn main() {
    assert_eq!(solution(vec![0, 0]), "0");
    assert_eq!(solution(vec![10, 2]), "210");
    assert_eq!(solution(vec![3, 30, 34, 5, 9]), "9534330");
}
