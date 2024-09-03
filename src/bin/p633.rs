fn solution(c: i32) -> bool {
    fn is_square(n: i32) -> bool {
        let sqrt_n = ((n as f32).sqrt() + 0.5) as i32;
        sqrt_n * sqrt_n == n
    }

    // we start with some observations. to make things simple, we assume
    // that `a <= b`.
    // immediately we have:
    // - `a^2 < a^2 + b^2 = c < 2^31 - 1`,
    // - `2b^2 < a^2 + b^2 = c < 2^31 - 1`
    // yielding `a <= 32767` and `b <= 46340`.
    // so we just need to loop over `a` in `0..=32767` and check whether
    // `c - a^2` is a square number.
    const MAX_A: i32 = 32767;
    for a in (0..=MAX_A).rev() {
        if is_square(c - a * a) {
            return true;
        }
    }
    false
}

fn main() {
    assert!(solution(5));
    assert!(!solution(3));
}
