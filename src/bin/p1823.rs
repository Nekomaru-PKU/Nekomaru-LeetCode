mod solution {
    pub fn main(n: i32, k: i32) -> i32 {
        let mut next = (1..=n as usize).collect::<Vec<_>>();
        next[n as usize - 1] = 0;
        let mut curr = 0;
        let mut prev = n as usize - 1;
        while next[curr] != curr {
            for _ in 0..k - 1 {
                (prev, curr) = (curr, next[curr]);
            }
            curr = next[curr];
            next[prev] = curr;
        }
        curr as i32 + 1
    }
}

fn main() {
    assert_eq!(solution::main(5, 2), 3);
    assert_eq!(solution::main(6, 5), 1);
}
