mod math {
    use core::ops;

    pub fn gcd<T>(mut a: T, mut b: T) -> T
    where
        T: Default + Copy + Ord + ops::Rem<Output = T> {
        let zero = T::default();
        debug_assert!(a > zero);
        debug_assert!(b > zero);
        if    a < b    { (a, b) = (b, a); }
        while b > zero { (a, b) = (b, a % b); }
        a
    }

    pub fn gcd_ext<T>(a: T, b: T) -> T
    where
        T: Default + Copy + Ord
            + ops::Neg<Output = T>
            + ops::Rem<Output = T> {
        use core::cmp::Ordering::{Equal, Less, Greater};
        let zero = T::default();
        match (a.cmp(&zero), b.cmp(&zero)) {
            (Equal, Equal) => panic!("gcd(0, 0) is undefined"),
            (Equal, _) => b,
            (_, Equal) => a,
            (Less, Less) => self::gcd(-a, -b),
            (Less, Greater) => self::gcd(-a, b),
            (Greater, Less) => self::gcd(a, -b),
            (Greater, Greater) => self::gcd(a, b),
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
        let gcd = self::gcd_ext(a, b);
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
            _ => panic!(),
        }
    }

    acc = math::frac_add(acc, {
        let State::NumDenum(num, denum) = state else {
            panic!();
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
