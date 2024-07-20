mod solution {
    pub fn main(row_sum: Vec<i32>, column_sum: Vec<i32>) -> Vec<Vec<i32>> {
        use std::iter;

        let num_rows = row_sum.len();
        let num_columns = column_sum.len();
        let mut num_not_solved = num_rows + num_columns;
        let mut row_sum = row_sum;
        let mut column_sum = column_sum;

        let mut matrix = iter::repeat(
            iter::repeat(0)
                .take(num_columns)
                .collect::<Vec<_>>())
            .take(num_rows)
            .collect::<Vec<_>>();

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        enum ConstraintTarget {
            Row(usize),
            Column(usize),
        }

        while let Some((target, sum)) =
            if num_not_solved > 1 {
                [
                    (&row_sum   , ConstraintTarget::Row    as fn(_) -> _),
                    (&column_sum, ConstraintTarget::Column as fn(_) -> _)
                ]   .into_iter()
                    .flat_map(|(sum, target)| sum.iter()
                        .enumerate()
                        .map(move |(idx, &sum)| (target(idx), sum)))
                    .filter(|&(_, sum)| sum != 0)
                    .min_by_key(|&(_, sum)| sum)
            } else {None} {
            println!("{:?}", (target, sum));

            let (i, j) = match target {
                ConstraintTarget::Row(i) =>
                    iter::zip(iter::repeat(i), 0..num_columns)
                        .find(|&(_, j)| column_sum[j] != 0),
                ConstraintTarget::Column(j) =>
                    iter::zip(0..num_rows, iter::repeat(j))
                        .find(|&(i, _)| row_sum[i] != 0)
            }.unwrap();

            matrix[i][j] = sum;
            row_sum[i] -= sum;
            column_sum[j] -= sum;
            num_not_solved -= 1;
        }

        matrix
    }
}

fn main() {
    assert_eq!(solution::main(vec![3, 8], vec![4, 7]), vec![vec![3, 0], vec![1, 7]]);
}
