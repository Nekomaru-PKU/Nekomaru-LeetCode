mod shortest_path {
    #![expect(clippy::needless_range_loop)]

    pub fn floyd<E>(n: usize, edges: E) -> Vec<Vec<i32>>
    where
        E: Iterator<Item = (usize, usize, i32)> {
        let mut dist = vec![vec![i32::MAX; n]; n];
        for i in 0..n {
            dist[i][i] = 0;
        }
        for (i, j, weight) in edges {
            dist[i][j] = dist[i][j].min(weight);
        }
        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    dist[i][j] = dist[i][j].min(
                        i32::saturating_add(
                            dist[i][k],
                            dist[k][j]));
                }
            }
        }
        dist
    }
}

fn solution(
    source: String,
    target: String,
    original: Vec<char>,
    changed: Vec<char>,
    cost: Vec<i32>)
 -> i64 {
    let dist = shortest_path::floyd(
        26,
        Iterator::zip(original.iter(), changed.iter())
            .zip(&cost)
            .map(|((&from, &to), &cost)| (
                from as usize - 'a' as usize,
                to   as usize - 'a' as usize,
                cost)));

    let mut total_cost = 0;
    for (from, to) in Iterator::zip(
        source.chars(),
        target.chars()) {
        let cost = dist[
            from as usize - 'a' as usize][
            to   as usize - 'a' as usize
        ];
        if cost == i32::MAX {
            return -1;
        }
        total_cost += cost as i64;
    }
    total_cost
}

fn main() {
    assert_eq!(solution(
        "abcd".into(),
        "acbe".into(),
        "abcced".chars().collect(),
        "bcbebe".chars().collect(),
        vec![2, 5, 5, 1, 2, 20]),
        28);
}
