fn solution(num: i32) -> i32 {
    let mut mask = 1;
    while mask < num { mask = mask << 1 | 1; }
    num ^ mask
}

fn main() {
    assert_eq!(solution(5), 2);
    assert_eq!(solution(1), 0);
    assert_eq!(solution(i32::MAX), 0);
}
