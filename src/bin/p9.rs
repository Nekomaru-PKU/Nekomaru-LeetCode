const fn solution(x: i32) -> bool {
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

    if x < 0 {
        false
    } else if x == 0 {
        true
    } else if x >= POW_OF_10[9] {
        x / POW_OF_10[9] == x % 10
            && x / POW_OF_10[8] % 10 == x / POW_OF_10[1] % 10
            && x / POW_OF_10[7] % 10 == x / POW_OF_10[2] % 10
            && x / POW_OF_10[6] % 10 == x / POW_OF_10[3] % 10
            && x / POW_OF_10[5] % 10 == x / POW_OF_10[4] % 10
    } else if x >= POW_OF_10[8] {
        x / POW_OF_10[8] == x % 10
            && x / POW_OF_10[7] % 10 == x / POW_OF_10[1] % 10
            && x / POW_OF_10[6] % 10 == x / POW_OF_10[2] % 10
            && x / POW_OF_10[5] % 10 == x / POW_OF_10[3] % 10
    } else if x >= POW_OF_10[7] {
        x / POW_OF_10[7] == x % 10
            && x / POW_OF_10[6] % 10 == x / POW_OF_10[1] % 10
            && x / POW_OF_10[5] % 10 == x / POW_OF_10[2] % 10
            && x / POW_OF_10[4] % 10 == x / POW_OF_10[3] % 10
    } else if x >= POW_OF_10[6] {
        x / POW_OF_10[6] == x % 10
            && x / POW_OF_10[5] % 10 == x / POW_OF_10[1] % 10
            && x / POW_OF_10[4] % 10 == x / POW_OF_10[2] % 10
    } else if x >= POW_OF_10[5] {
        x / POW_OF_10[5] == x % 10
            && x / POW_OF_10[4] % 10 == x / POW_OF_10[1] % 10
            && x / POW_OF_10[3] % 10 == x / POW_OF_10[2] % 10
    } else if x >= POW_OF_10[4] {
        x / POW_OF_10[4] == x % 10
            && x / POW_OF_10[3] % 10 == x / POW_OF_10[1] % 10
    } else if x >= POW_OF_10[3] {
        x / POW_OF_10[3] == x % 10
            && x / POW_OF_10[2] % 10 == x / POW_OF_10[1] % 10
    } else if x >= POW_OF_10[2] {
        x / 100 == x % 10
    } else if x >= POW_OF_10[1] {
        x / 10 == x % 10
    } else {
        true
    }
}

fn main() {
    assert!(solution(121));
    assert!(!solution(-121));
    assert!(!solution(10));
    assert!(!solution(i32::MIN));
    assert!(!solution(i32::MAX));
    assert!(solution(2_147_447_412));
}
