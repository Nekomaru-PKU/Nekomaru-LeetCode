fn solution(skill: Vec<i32>) -> i64 {
    let mut cnt = [0u32; 1024];
    let mut sum = 0;
    let mut max = 0;
    for &val in &skill {
        let val = val as usize;
        cnt[val] += 1;
        sum += val;
        max = max.max(val);
    }

    let cnt = cnt;
    let sum = sum;
    let max = max;

    let num_teams = skill.len() / 2;
    if sum % num_teams != 0 {
        return -1;
    }

    let sum_per_team = sum / num_teams;
    if max > sum_per_team {
        return -1;
    }

    let mut acc = 0;
    if sum_per_team % 2 == 0 {
        let mid_val = sum_per_team / 2;
        let mid_cnt = cnt[mid_val];
        if mid_cnt % 2 == 0 {
            acc +=
                ((mid_cnt / 2) as i64) *
                (mid_val as i64) *
                (mid_val as i64);
        } else {
            return -1;
        }
    }

    for lval in 0..=((sum_per_team - 1) / 2) {
        let rval = sum_per_team - lval;
        let lcnt = cnt.get(lval).copied().unwrap_or_default();
        let rcnt = cnt.get(rval).copied().unwrap_or_default();
        if  lcnt == rcnt {
            acc +=
                (lcnt as i64) *
                (lval as i64) *
                (rval as i64);
        } else {
            return -1;
        }
    }

    acc
}

fn main() {
    assert_eq!(solution(vec![3, 2, 5, 1, 3, 4]), 22);
    assert_eq!(solution(vec![3, 4]), 12);
    assert_eq!(solution(vec![1, 1, 2, 3]), -1);
    assert_eq!(solution(vec![1000, 1000]), 1_000_000);
}
