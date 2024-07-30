fn solution(s: String) -> i32 {
    // <time: O(n), space: O(n)>

    let s = s.into_bytes();
    let n = s.len();

    // we precompute `m[i]` as the number of 'a's in `s[..i]`.
    let mut m = vec![0; n + 1];
    for i in 1..=n {
        m[i] = m[i - 1] + if s[i - 1] == b'a' {1} else {0};
    }

    (0..=n)
        // in the i-th iteration, we check the case where we delete
        // all 'b's in `s[..i]` and all 'a's in `s[i..]`,
        // as we know that:
        // - `s[..i]` contains `i   - m_i` 'b's,
        // - `s[i..]` contains `m_n - m_i` 'a's.
        // we need `(i - m_i) + (m_n - m_i)` deletions in such case.
        .map(|i| (i as i32 - m[i]) + (m[n] - m[i]))
        .min()
        .unwrap()
}

fn solution_optimized(s: String) -> i32 {
    // <time: O(n), space: O(1)>
    // this is a space-optimized version of the same algorithm.

    let s = s.into_bytes();
    let n = s.len();

    let mut min = i32::MAX;
    let mut m_i = 0;
    for i in 0..=n {
        // we use `m_i` to track the number of 'a's in `s[..i]`
        if i > 0 && s[i - 1] == b'a' {
            m_i += 1;
        }

        // in the i-th iteration, we check the case where we delete
        // all 'b's in `s[..i]` and all 'a's in `s[i..]`,
        // as we know that:
        //     `s[..i]` contains `i   - m_i` 'b's,
        //     `s[i..]` contains `m_n - m_i` 'a's.
        // we need `(i - m_i) + (m_n - m_i)` deletions in such case.
        // 
        // since we do not know `m_n` until the end of all iterations,
        // we track the minimal of `i - 2 * m_i` instead.
        min = min.min(i as i32 - 2 * m_i);
    }

    min + m_i
}

fn main() {
    let _ = solution("".into());
    assert_eq!(solution_optimized("aababbab" .into()), 2);
    assert_eq!(solution_optimized("bbaaaaabb".into()), 2);
}
