fn solution(customers: Vec<i32>, grumpy: Vec<i32>, minutes: i32) -> i32 {
    let mut num_satisfied =
        customers.iter()
            .take(minutes as _)
            .sum::<i32>() +
        customers.iter()
            .zip(&grumpy)
            .skip(minutes as _)
            .filter(|(_, &grumpy)| grumpy == 0)
            .map(|(&customers, _)| customers)
            .sum::<i32>();
    let mut max_satisfied = num_satisfied;
    for begin in 1..=(customers.len() - minutes as usize) {
        let end = begin + minutes as usize;
        num_satisfied -= grumpy[begin - 1] * customers[begin - 1];
        num_satisfied += grumpy[end - 1] * customers[end - 1];
        max_satisfied = max_satisfied.max(num_satisfied);
    }
    max_satisfied
}

fn main() {
    assert_eq!(
        solution(
            vec![1, 0, 1, 2, 1, 1, 7, 5],
            vec![0, 1, 0, 1, 0, 1, 0, 1],
            3),
        16);
    assert_eq!(solution(vec![1], vec![0], 1), 1);
    assert_eq!(solution(vec![4, 10, 10], vec![1, 1, 0], 2), 24);
}
