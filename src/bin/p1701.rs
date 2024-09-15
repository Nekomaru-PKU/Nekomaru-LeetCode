fn solution(customers: Vec<Vec<i32>>) -> f64 {
    let mut now = 0;
    let mut sum = 0;
    for customer in &customers {
        let arrival = customer[0];
        let time = customer[1];
        sum += ((now - arrival).max(0) + time) as i64;
        now = now.max(arrival) + time;
    }
    sum as f64 / customers.len() as f64
}

fn main() {
    #![expect(clippy::float_cmp)]
    use leetcode::input::Input;
    assert_eq!(solution([[1, 2], [2, 5], [4, 3]].input()), 5.0);
    assert_eq!(solution([[5, 2], [5, 4], [10, 3], [20, 1]].input()), 3.25);
}
