mod solution {
    use super::iter;

    pub fn main(grid: Vec<Vec<i32>>) -> i32 {
        let rows = grid.len();
        let cols = grid.first().unwrap().len();
        if rows >= 3 && cols >= 3 {
            iter::product(0..=rows - 3, 0..=cols - 3)
                .filter(|&(i, j)| is_magic_square(&grid, i, j))
                .count() as _
        } else {0}
    }

    fn is_magic_square(
        grid: &[Vec<i32>],
        i: usize,
        j: usize) -> bool {
        iter::product(0..3, 0..3)
            .map(|(ii, jj)| 1u16 << (grid[i + ii][j + jj] - 1))
            .sum::<u16>() == 0x01FF &&
        (0..3).all(|ii| (0..3)
            .map(|jj| grid[i + ii][j + jj])
            .sum::<i32>() == 15) &&
        (0..3).all(|jj| (0..3)
            .map(|ii| grid[i + ii][j + jj])
            .sum::<i32>() == 15) &&
        (0..3)
            .map(|kk| grid[i + kk][j + kk])
            .sum::<i32>() == 15 &&
        (0..3)
            .map(|kk| grid[i + kk][j + 2 - kk])
            .sum::<i32>() == 15
    }
}

mod iter {
    pub fn product<T0, T1, I0, I1>(i0: I0, i1: I1)
     -> impl Iterator<Item = (T0, T1)>
    where
        T0: Clone,
        I0: Iterator<Item = T0>,
        I1: Iterator<Item = T1> + Clone {
        i0.flat_map(move |v0|
            i1.clone().map(move |v1| (v0.clone(), v1)))
    }
}

fn main() {
    assert_eq!(solution::main(vec![
        vec![4, 3, 8, 4],
        vec![9, 5, 1, 9],
        vec![2, 7, 6, 2]]), 1);
    assert_eq!(solution::main(vec![vec![8]]), 0);
}
