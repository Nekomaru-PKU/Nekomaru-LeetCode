fn solution(x: i32) -> i32 {
    if x == 0 || x == i32::MIN { return 0; }
    if x < 0 { return -solution(-x) };

    const POW_OF_10: [i32; 10] = [
        1,
        10,
        100,
        1_000,
        10_000,
        100_000,
        1_000_000,
        10_000_000,
        100_000_000,
        1_000_000_000,
    ];

    #[allow(clippy::suspicious_else_formatting)]
    let num_digits = 1 +
        if x >= POW_OF_10[9] {9} else
        if x >= POW_OF_10[8] {8} else
        if x >= POW_OF_10[7] {7} else
        if x >= POW_OF_10[6] {6} else
        if x >= POW_OF_10[5] {5} else
        if x >= POW_OF_10[4] {4} else
        if x >= POW_OF_10[3] {3} else
        if x >= POW_OF_10[2] {2} else
        if x >= POW_OF_10[1] {1} else {0};

    let mut x = x;
    let mut y = 0i32;
    for i in (0..num_digits).rev() {
        let div = x / 10;
        let rem = x % 10;
        x = div;
        y = y.saturating_add(rem.saturating_mul(POW_OF_10[i]));
    }
    if y < i32::MAX {y} else {0}
}

fn main() {
    assert_eq!(solution(123), 321);
    assert_eq!(solution(-123), -321);
    assert_eq!(solution(120), 21);

    assert_eq!(solution(1_234_543_210), 123_454_321);
    assert_eq!(solution(-1_234_543_210), -123_454_321);

    assert_eq!(solution(2_123_454_321), 1_234_543_212);
    assert_eq!(solution(-2_123_454_321), -1_234_543_212);

    assert_eq!(solution(1_234_565_432), 0);
    assert_eq!(solution(-1_234_565_432), 0);

    assert_eq!(solution(i32::MIN), 0);
}
