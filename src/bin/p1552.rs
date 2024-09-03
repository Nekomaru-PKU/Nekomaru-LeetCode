mod solution {
    pub fn main(mut pos: Vec<i32>, m: i32) -> i32 {
        if (pos.len() as i32) < m {
            return 0;
        }

        pos.sort_unstable();
        let pos_min = pos[0];
        let pos_max = pos[pos.len() - 1];

        let mut max = (pos_max - pos_min) / (m - 1);
        let mut min = 1;
        while min != max {
            let mid = min + (max - min + 1) / 2;
            if check(&pos, m, mid) {
                min = mid;
            } else {
                max = mid - 1;
            }
        }
        min
    }

    fn check(pos: &[i32], m: i32, dist: i32) -> bool {
        // we always put a ball into the first basket.
        let mut ball = 1;
        let mut prev = pos[0];
        for &pos in &pos[1..] {
            if pos - prev >= dist {
                prev = pos;
                ball += 1;
                if ball >= m {
                    return true;
                }
            }
        }
        false
    }
}

fn main() {
    assert_eq!(solution::main(vec![1, 2, 3, 4, 7], 3), 3);
    assert_eq!(solution::main(vec![5, 4, 3, 2, 1, 1_000_000_000], 2), 999_999_999);
}
