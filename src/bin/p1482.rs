mod solution {
    pub fn main(bloom_day: Vec<i32>, m: i32, k: i32) -> i32 {
        if bloom_day.len() < (m as usize) * (k as usize) {
            return -1;
        }

        let mut min = 0;
        let mut max = bloom_day.iter().copied().max().expect("n >= 1");
        while min != max {
            let mid = min + (max - min) / 2;
            if check(&bloom_day, m, k, mid) {
                max = mid;
            } else {
                min = mid + 1;
            }
        }
        min
    }

    fn check(bloom_day: &[i32], m: i32, k: i32, t: i32) -> bool {
        let mut adjacent = 0;
        let mut bouquets = 0;
        for &d in bloom_day {
            if d <= t {
                adjacent += 1;
                if adjacent >= k {
                    adjacent = 0;
                    bouquets += 1;
                    if bouquets >= m {
                        return true;
                    }
                }
            } else {
                adjacent = 0;
            }
        }
        false
    }
}

fn main() {
    assert_eq!(solution::main(vec![1, 10, 3, 10, 2], 3, 1), 3);
    assert_eq!(solution::main(vec![1, 10, 3, 10, 2], 3, 2), -1);
    assert_eq!(solution::main(vec![7, 7, 7, 7, 12, 7, 7], 2, 3), 12);
}
