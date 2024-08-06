fn solution(s: String) -> i32 {
    use std::array;

    let s = s.into_bytes();

    let mut freq = [0; 26];
    for &c in &s {
        freq[(c - b'a') as usize] += 1;
    }

    let mut sorted: [_; 26] = array::from_fn(|c| c);
    sorted.sort_unstable_by_key(|&c| -freq[c]);

    let mut pushes = [0; 26];
    for (i, &c) in sorted.iter().enumerate() {
        pushes[c] = i as i32 / 8 + 1;
    }

    s.iter().map(|&c| pushes[(c - b'a') as usize]).sum()
}

fn main() {
    assert_eq!(solution("abcde".into()), 5);
    assert_eq!(solution("xyzxyzxyzxyz".into()), 12);
    assert_eq!(solution("aabbccddeeffgghhiiiiii".into()), 24);
}
