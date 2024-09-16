fn solution(time_points: Vec<String>) -> i32 {
    let n@0..=1440 = time_points.len() else {
        return 0;
    };
    let mut t =
        time_points
            .into_iter()
            .map(String::into_bytes)
            .map(|s| {
                (s[0] as i32 * 10 + s[1] as i32) * 60 +
                (s[3] as i32 * 10 + s[4] as i32)
            })
            .collect::<Vec<_>>();
    t.sort_unstable();
    t.push(t[0] + 24 * 60);
    (0..n)
        .map(|i| t[i + 1] - t[i])
        .min()
        .unwrap()
}

fn main() {
    use leetcode::input::Input;
    assert_eq!(solution(["23:59","00:00"].input()), 1);
    assert_eq!(solution(["00:00","23:59","00:00"].input()), 0);
}