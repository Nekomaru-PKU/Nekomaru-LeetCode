fn solution(n: i32, roads: Vec<Vec<i32>>) -> i64 {
    let mut degrees = vec![0; n as _];
    for road in &roads {
        degrees[road[0] as usize] += 1;
        degrees[road[1] as usize] += 1;
    }

    let mut cities_sorted = degrees.iter()
        .cloned()
        .enumerate()
        .map(|(i, d)| (i as i32, d))
        .collect::<Vec<_>>();
    cities_sorted.sort_unstable_by_key(|&(_, d)| d);

    let mut values = vec![0; n as _];
    for (i, &(city, _)) in cities_sorted.iter().enumerate() {
        values[city as usize] = i as i32 + 1;
    }

    roads.iter()
        .map(|road| (
            values[road[0] as usize] +
            values[road[1] as usize]) as i64)
        .sum::<i64>()
}

fn main() {
    use leetcode::input::Input;
    assert_eq!(solution(5, [[0, 1], [1, 2], [2, 3], [0, 2], [1, 3], [2, 4]].input()), 43);
    assert_eq!(solution(5, [[0, 3], [2, 4], [1, 3]].input()), 20);
}
