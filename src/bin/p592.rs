mod math {
    use std::{
        ops,
        cmp::Ordering,
    };

    pub fn gcd<T>(a: T, b: T) -> T
    where
        T: Default + Copy + Ord
            + ops::Neg<Output = T>
            + ops::Rem<Output = T> {
        let zero = T::default();
        match (a.cmp(&zero), b.cmp(&zero)) {
            (Ordering::Equal, _) => b,
            (_, Ordering::Equal) => a,
            (Ordering::Less, _) => gcd(-a, b),
            (_, Ordering::Less) => gcd(a, -b),
            (Ordering::Greater, Ordering::Greater) =>
                match T::cmp(&a, &b) {
                    Ordering::Less => gcd(b, a),
                    Ordering::Equal => a,
                    Ordering::Greater => {
                        let (mut a, mut b) = (a, b);
                        while b > zero {
                            (a, b) = (b, a % b);
                        }
                        a
                    }
                },
        }
    }

    pub fn frac_add<T>(
        (a0, b0): (T, T),
        (a1, b1): (T, T)) -> (T, T)
    where
        T: Default + Copy + Ord
            + ops::Neg<Output = T>
            + ops::Add<Output = T>
            + ops::Mul<Output = T>
            + ops::Div<Output = T>
            + ops::Rem<Output = T> {
        let a = a0 * b1 + a1 * b0;
        let b = b0 * b1;
        let gcd = gcd(a, b);
        (a / gcd, b / gcd)
    }
}

pub fn solution(s: String) -> String {
    enum State {
        None,
        Neg,
        Num(i64),
        NumDenum(i64, i64),
    }

    let mut acc = (0, 1);
    let mut state = State::None;
    for c in s.into_bytes() {
        state = match (c, state) {
            (b'0'..=b'9', State::None) =>
                State::Num((c - b'0') as _),
            (b'0'..=b'9', State::Neg) =>
                State::Num(-((c - b'0') as i64)),
            (b'0'..=b'9', State::Num(num)) =>
                State::Num(num * 10 + ((c - b'0') as i64)),
            (b'0'..=b'9', State::NumDenum(num, denum)) =>
                State::NumDenum(num, denum * 10 + ((c - b'0') as i64)),
            (b'/', State::Num(num)) =>
                State::NumDenum(num, 0),
            (b'+', State::None) => State::None,
            (b'+', State::NumDenum(num, denum)) => {
                acc = math::frac_add(acc, (num, denum));
                State::None
            },
            (b'-', State::None) => State::Neg,
            (b'-', State::NumDenum(num, denum)) => {
                acc = math::frac_add(acc, (num, denum));
                State::Neg
            },
            _ => unreachable!(),
        }
    }

    acc = math::frac_add(acc, {
        let State::NumDenum(num, denum) = state else {
            unreachable!();
        };
        (num, denum)
    });

    use std::fmt::Write;
    let mut out = String::new();
    if acc.0 < 0 { out.push('-'); }
    let _ = write!(out, "{}/{}", acc.0.abs(), acc.1).err();
    out
}

fn main() {
    assert_eq!(solution("-1/2+1/2".into()), "0/1");
    assert_eq!(solution("-1/2+1/2+1/3".into()), "1/3");
    assert_eq!(solution("1/3-1/2".into()), "-1/6");
}
