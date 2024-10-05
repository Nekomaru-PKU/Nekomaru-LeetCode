fn solution(s1: String, s2: String) -> bool {
    let pat = s1.as_bytes();
    let str = s2.as_bytes();
    if  str.len() < pat.len() { return false; }

    fn index_of(c: u8) -> usize {
        debug_assert!(c.is_ascii_lowercase());
        (c - b'a') as usize
    }

    let mut pat_cnt = [0u16; 26];
    for &c in pat {
        pat_cnt[index_of(c)] += 1;
    }
    let pat_cnt = pat_cnt;

    let mut str_cnt = [0u16; 26];
    for &c in &str[..pat.len()] {
        str_cnt[index_of(c)] += 1;
    }
    if str_cnt == pat_cnt {
        return true;
    }

    for begin in 0..(str.len() - pat.len()) {
        str_cnt[index_of(str[begin])] -= 1;
        str_cnt[index_of(str[begin + pat.len()])] += 1;
        if str_cnt == pat_cnt {
            return true;
        }
    }
    false
}

fn main() {
    assert!(solution("ab".into(), "eidbaooo".into()));
    assert!(!solution("ab".into(), "eidboaoo".into()));
}
