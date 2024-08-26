fn solution(s: String) -> i32 {
    let mut s = s.into_bytes();
    s.reverse();
    let mut num = 0;
    while let Some(c) = s.pop() {
        match c {
            b'M' => num += 1000,
            b'D' => num += 500,
            b'C' => match s.last() {
                Some(b'M') => { s.pop(); num += 900; }
                Some(b'D') => { s.pop(); num += 400; }
                _ => num += 100,
            },
            b'L' => num += 50,
            b'X' => match s.last() {
                Some(b'C') => { s.pop(); num += 90; }
                Some(b'L') => { s.pop(); num += 40; }
                _ => num += 10,
            },
            b'V' => num += 5,
            b'I' => match s.last() {
                Some(b'X') => { s.pop(); num += 9; }
                Some(b'V') => { s.pop(); num += 4; }
                _ => num += 1,
            },
            _ => unreachable!()
        }
    }
    num
}

fn main() {
    assert_eq!(solution("III".into()), 3);
    assert_eq!(solution("LVIII".into()), 58);
    assert_eq!(solution("MCMXCIV".into()), 1994);
}
