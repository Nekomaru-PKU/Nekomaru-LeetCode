fn solution(n: i32) -> Vec<i32> {
    let mut out = Vec::with_capacity(n as _);
    let mut m = 1;
    let mut going_down = true;
    while m != 0 {
        if going_down {
            println!("n: {n}, m: {m}");
            out.push(m);
            if m * 10 <= n {
                m *= 10;
            } else {
                going_down = false;
            }
        } else if m % 10 != 9 && m < n {
            m += 1;
            going_down = true;
        } else {
            m /= 10;
        }
    }
    out
}

fn main() {
    for n in [90, 99, 900, 990, 999, 1000] {
        let mut expected = (1..=n).collect::<Vec<_>>();
        expected.sort_unstable_by_key(i32::to_string);
        assert_eq!(solution(n), expected);
    }
}
