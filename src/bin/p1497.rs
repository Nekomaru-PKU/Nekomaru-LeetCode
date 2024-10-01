fn solution(arr: Vec<i32>, k: i32) -> bool {
    if k == 1 { return true; }
    let cnt = arr.iter().fold(vec![0; k as _], |mut cnt, &num| {
        let rem = num.rem_euclid(k as _);
        cnt[rem as usize] += 1;
        cnt
    });
    let k = k as usize;
    cnt[0] % 2 == 0 && if k % 2 != 0 {
        // for example, if k = 3,
        // this checks for cnt[1] == cnt[2].
        (1..=(k / 2)).all(|i| cnt[i] == cnt[k - i])
    } else {
        // for example, if k = 4,
        // this checks for cnt[1] == cnt[3].
        (1.. (k / 2)).all(|i| cnt[i] == cnt[k - i]) &&
        // and this checks for cnt[2] is even.
        cnt[k / 2] % 2 == 0
    }
}

fn main() {
    assert!(solution((1..=10).collect(), 5));
    assert!(solution((1..=6).collect(), 7));
    assert!(!solution((1..=6).collect(), 10));
}
