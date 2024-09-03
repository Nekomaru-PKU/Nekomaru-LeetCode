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
                (i > 0    ).then(|| (i - 1, j)),
                (i < n - 1).then(|| (i + 1, j)),
                (j > 0    ).then(|| (i, j - 1)),
                (j < m - 1).then(|| (i, j + 1)),
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
    use leetcode::input::Input;
    assert_eq!(solution([[1]].input(), [[0]].input()), 0);
    assert_eq!(solution(
        [
            [1, 1, 1, 0, 0],
            [0, 1, 1, 1, 1],
            [0, 0, 0, 0, 0],
            [1, 0, 0, 0, 0],
            [1, 1, 0, 1, 1],
        ].input(),
        [
            [1, 1, 1, 0, 0],
            [0, 0, 1, 1, 1],
            [0, 1, 0, 0, 0],
            [1, 0, 1, 1, 0],
            [0, 1, 0, 1, 0],
        ].input()), 3);
    assert_eq!(solution(
        [
            [1, 0, 1, 0, 1],
            [1, 1, 1, 1, 1],
            [0, 0, 0, 0, 0],
            [1, 1, 1, 1, 1],
            [1, 0, 1, 0, 1],
        ].input(),
        [
            [0, 0, 0, 0, 0],
            [1, 1, 1, 1, 1],
            [0, 1, 0, 1, 0],
            [0, 1, 0, 1, 0],
            [1, 0, 0, 0, 1],
        ].input()), 2);
}
