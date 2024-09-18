mod iter {
    pub use core::iter::*;

    pub fn product<T0, T1, I0, I1>(outer: I0, inner: I1)
     -> impl Iterator<Item = (T0, T1)>
    where
        T0: Clone,
        I0: IntoIterator<Item = T0>,
        I1: IntoIterator<Item = T1> + Clone {
        outer.into_iter().flat_map(move |v0| {
            inner.clone().into_iter().map(move |v1| {
                (v0.clone(), v1)
            })
        })
    }
}

fn solution(grid: Vec<Vec<i32>>) -> i32 {
    use core::array;
    let num_rows = grid.len();
    let num_cols = grid.first().map(Vec::len).unwrap_or_default();
    let rows_cols_diagnols =
        iter::empty()
            .chain((0..3).map(|ii| array::from_fn::<_, 3, _>(|jj| (ii, jj))))
            .chain((0..3).map(|jj| array::from_fn::<_, 3, _>(|ii| (ii, jj))))
            .chain(iter::once(array::from_fn::<_, 3, _>(|kk| (kk, kk))))
            .chain(iter::once(array::from_fn::<_, 3, _>(|kk| (kk, 2 - kk))))
            .collect::<Vec<_>>();
    (num_rows >= 3 && num_cols >= 3).then(|| {
        iter::product(0..=(num_rows - 3), 0..=(num_cols - 3))
            .filter(|&(i, j)| {
                iter::product(0..3, 0..3)
                    .map(|(ii, jj)| 1u16 << (grid[i + ii][j + jj] - 1))
                    .sum::<u16>() == 0x01FF &&
                rows_cols_diagnols.iter().all(|cond| {
                    cond.iter()
                        .map(|&(ii, jj)| grid[i + ii][j + jj])
                        .sum::<i32>() == 15
                })
            })
            .count() as _
    }).unwrap_or_default()
}

fn main() {
    assert_eq!(solution(vec![
        vec![4, 3, 8, 4],
        vec![9, 5, 1, 9],
        vec![2, 7, 6, 2],
    ]), 1);
    assert_eq!(solution(vec![vec![8]]), 0);
}
