fn solution(piles: Vec<i32>) -> i32 {
    let n = piles.len();
    let m_max = (n + 1) / 2;
    let sum = {
        let mut sum = vec![0; n + 1];
        let mut acc = 0;
        for i in 0..n {
            acc += piles[i];
            sum[i + 1] = acc;
        }
        sum
    };

    #[derive(Default, Clone, Copy)]
    struct Act {
        take: usize,
        score: i32,
    }

    let mut best_act_on = vec![vec![Act::default(); m_max + 1]; n + 1];
    for my_start in (0..n).rev() {
        for m in 1..=m_max {
            let mut my_best_act = Act::default();
            for my_take in 1..=(2 * m).min(n - my_start) {
                let my_score = sum[my_start + my_take] - sum[my_start];

                let opponent_start = my_start + my_take;
                let opponent_m = m.max(my_take).min(m_max);
                let opponent = best_act_on[opponent_start][opponent_m];

                let my_next_start = opponent_start + opponent.take;
                let my_next_m = opponent_m.max(opponent.take).min(m_max);
                let my_next = best_act_on[my_next_start][my_next_m];

                if my_score + my_next.score > my_best_act.score {
                    my_best_act.take = my_take;
                    my_best_act.score = my_score + my_next.score;
                }
            }
            best_act_on[my_start][m] = my_best_act;
        }
    }

    best_act_on[0][1].score
}

fn main() {
    assert_eq!(solution(vec![1]), 1);
    assert_eq!(solution(vec![2, 7, 9, 4, 4]), 10);
    assert_eq!(solution(vec![1, 2, 3, 4, 5, 100]), 104);
}
