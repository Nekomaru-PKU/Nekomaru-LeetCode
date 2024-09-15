fn solution(s: String) -> i32 {
    // [time: O(n^2), space: O(n)]
    use std::{
        iter,
        ops::BitXorAssign,
    };
    let s = s.into_bytes();
    let acc = iter::once(0)
        .chain(s.iter().scan(0, |acc, &c| {
            acc.bitxor_assign(match c {
                b'a' => 1,
                b'e' => 1 << 1,
                b'i' => 1 << 2,
                b'o' => 1 << 3,
                b'u' => 1 << 4,
                _ => 0,
            });
            Some(*acc)
        }))
        .collect::<Vec<_>>();
    (0..=s.len())
        .rev()
        .find(|substr_len| {
            (0..=(s.len() - substr_len)).any(|begin| {
                acc[begin + substr_len] ^ acc[begin] == 0
            })
        })
        .unwrap() as _
}

fn main() {
    assert_eq!(solution("eleetminicoworoep".into()), 13);
    assert_eq!(solution("leetcodeisgreat".into()), 5);
    assert_eq!(solution("bcbcbc".into()), 6);
    assert_eq!(solution("amntyyaw".into()), 8);
}
