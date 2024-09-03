mod solution {
    use std::collections::HashSet;

    pub fn main(mut grid: Vec<Vec<i32>>) -> i32 {
        let num_rows = grid.len();
        let num_cols = grid[0].len();
        if num_islands(&grid) != 1 {
            return 0;
        }

        for i in 0..num_rows {
            for j in 0..num_cols {
                if grid[i][j] > 0 {
                    grid[i][j] = 0;
                    if num_islands(&grid) != 1 {
                        return 1;
                    }
                    grid[i][j] = 1;
                }
            }
        }

        2
    }

    fn num_islands(grid: &[Vec<i32>]) -> i32 {
        let num_rows = grid.len();
        let num_cols = grid[0].len();

        let mut unvisited = (0..num_rows)
            .flat_map(|i| (0..num_cols).map(move |j| (i, j)))
            .filter(|&(i, j)| grid[i][j] > 0)
            .collect::<HashSet<_>>();
        let mut stack = Vec::new();
        let mut num = 0;
        while let Some(start_cell) = unvisited.iter().next().copied() {
            num += 1;
            stack.push(start_cell);
            while let Some((i, j)) = stack.pop() {
                unvisited.remove(&(i, j));
                stack.extend([
                    (i > 0           ).then(|| (i - 1, j)),
                    (i < num_rows - 1).then(|| (i + 1, j)),
                    (j > 0           ).then(|| (i, j - 1)),
                    (j < num_cols - 1).then(|| (i, j + 1)),
                ]   .into_iter()
                    .flatten()
                    .filter(|&(i, j)| grid[i][j] > 0)
                    .filter(|c| unvisited.contains(c)));
            }
        }
        num
    }
}

fn main() {
    assert_eq!(solution::main(vec![
        vec![0, 1, 1, 0],
        vec![0, 1, 1, 0],
        vec![0, 0, 0, 0]
    ]), 2);
    assert_eq!(solution::main(vec![
        vec![0, 1, 0, 0],
        vec![0, 1, 1, 0],
        vec![0, 0, 0, 0]
    ]), 1);
    assert_eq!(solution::main(vec![
        vec![0, 1, 0, 0],
        vec![0, 0, 1, 0],
        vec![0, 0, 0, 0]
    ]), 0);
    assert_eq!(solution::main(vec![
        vec![1, 1]
    ]), 2);
    assert_eq!(solution::main(vec![
        vec![1]
    ]), 1);
}

