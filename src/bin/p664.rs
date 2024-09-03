#![expect(clippy::needless_range_loop)]

fn solution(s: String) -> i32 {
    let s = {
        let mut s_new = Vec::with_capacity(s.len());
        for c in s.bytes() {
            if s_new.last().copied() != Some(c) {
                s_new.push(c);
            }
        }
        s_new
    };

    let n = s.len();

    // dp
    // - index0: len
    // - index1: begin
    // - index2: free_char

    const fn dp_index2(c: Option<u8>) -> usize {
        if let Some(c) = c {
            (c - b'a') as _
        } else {26}
    }

    let mut dp = (0..=n)
        .map(|len| vec![[i32::MAX; 27]; n - len + 1])
        .collect::<Vec<_>>();
    for begin in 0..n {
        dp[0][begin].fill(0);
        dp[1][begin].fill(1);
        dp[1][begin][dp_index2(Some(s[begin]))] = 0;
    }

    for len in 2..=n {
        for begin in 0..=(n - len) {
            // free_char: none
            dp[len][begin][dp_index2(None)] = Ord::min(
                // print s[begin]
                // use `s[(begin + 1)..(begin + len)]`
                dp[len - 1][begin + 1][dp_index2(Some(s[begin]))],
                // print s[begin + len - 1]
                // use `s[begin..(begin + len - 1)]`
                dp[len - 1][begin][dp_index2(Some(s[begin + len - 1]))]
            ).saturating_add(1);

            for c in b'a'..=b'z' {
                dp[len][begin][dp_index2(Some(c))] =
                    dp[len][begin][dp_index2(None)].min(
                        (1..len)
                            .map(|inner_len| i32::saturating_add(
                                // use `s[begin..(begin + inner_len)]`
                                dp[inner_len][begin][dp_index2(Some(c))],
                                // use `s[(begin + inner_len)..(begin + len)]`
                                dp[len - inner_len][begin + inner_len][dp_index2(Some(c))]))
                            .min()
                            .unwrap_or(i32::MAX));
            }
        }
    }

    dp[n][0][dp_index2(None)]
}

fn main() {
    assert_eq!(solution("aba".into()), 2);
    assert_eq!(solution("aaabbb".into()), 2);
    assert_eq!(solution("baacdddaaddaaaaccbddbcabdaabdbbcdcbbbacbddcabcaaa".into()), 19);
}
