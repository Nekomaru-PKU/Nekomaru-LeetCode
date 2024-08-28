fn solution(grid1: Vec<Vec<i32>>, grid2: Vec<Vec<i32>>) -> i32 {
    use std::collections::HashSet;

    let n = grid2.len();
    let m = grid2[0].len();

    let mut unvisited = (0..n)
        .flat_map(|i| (0..m).map(move |j| (i, j)))
        .filter(|&(i, j)| grid2[i][j] > 0)
        .collect::<HashSet<_>>();
    let mut stack = Vec::new();
    let mut out = 0;
    while let Some(&start) = unvisited.iter().next() {
        stack.push(start);
        unvisited.remove(&start);
        let mut is_island_in_grid1 = true;
        while let Some((i, j)) = stack.pop() {
            is_island_in_grid1 &= grid1[i][j] > 0;
            for (i, j) in [
                if i > 0     {Some((i - 1, j))} else {None},
                if i < n - 1 {Some((i + 1, j))} else {None},
                if j > 0     {Some((i, j - 1))} else {None},
                if j < m - 1 {Some((i, j + 1))} else {None},
            ].into_iter().flatten() {
                if unvisited.contains(&(i, j)) {
                    unvisited.remove(&(i, j));
                    stack.push((i, j));
                }
            }
        }
        if is_island_in_grid1 {
            out += 1;
        }
    }

    out
}

fn main() {
    assert_eq!(solution(vec![vec![1]], vec![vec![0]]), 0);
    assert_eq!(solution(
        vec![
            vec![1, 1, 1, 0, 0],
            vec![0, 1, 1, 1, 1],
            vec![0, 0, 0, 0, 0],
            vec![1, 0, 0, 0, 0],
            vec![1, 1, 0, 1, 1],
        ],
        vec![
            vec![1, 1, 1, 0, 0],
            vec![0, 0, 1, 1, 1],
            vec![0, 1, 0, 0, 0],
            vec![1, 0, 1, 1, 0],
            vec![0, 1, 0, 1, 0],
        ]), 3);
    assert_eq!(solution(
        vec![
            vec![1, 0, 1, 0, 1],
            vec![1, 1, 1, 1, 1],
            vec![0, 0, 0, 0, 0],
            vec![1, 1, 1, 1, 1],
            vec![1, 0, 1, 0, 1],
        ],
        vec![
            vec![0, 0, 0, 0, 0],
            vec![1, 1, 1, 1, 1],
            vec![0, 1, 0, 1, 0],
            vec![0, 1, 0, 1, 0],
            vec![1, 0, 0, 0, 1],
        ]), 2);
}
