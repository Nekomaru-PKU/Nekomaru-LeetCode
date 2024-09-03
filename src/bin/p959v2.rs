mod solution {
    use super::graph;

    const LEFT: bool = false;
    const TOP : bool = true;

    pub fn main(grid: Vec<String>) -> i32 {
        let grid = grid
            .into_iter()
            .map(String::into_bytes)
            .collect::<Vec<_>>();

        let num_rows = grid.len() as u8;
        let num_cols = grid[0].len() as u8;

        graph::num_connected_components(
            (0..num_rows)
                .flat_map(|i| (0..num_cols).map(move |j| (i, j))
                .flat_map(|(i ,j)| [
                    (i, j, LEFT),
                    (i, j, TOP),
                ]))
                .chain((0..num_rows).map(|i| (i, num_cols, LEFT)))
                .chain((0..num_cols).map(|j| (num_rows, j, TOP))),
            |(i, j, side)| successors(
                |i, j| grid[i as usize][j as usize],
                num_rows,
                num_cols,
                i,
                j,
                side)) as _
    }

    fn successors(
        get_cell: impl Fn(u8, u8) -> u8,
        num_rows: u8,
        num_cols: u8,
        i: u8,
        j: u8,
        side: bool)
     -> impl Iterator<Item = (u8, u8, bool)> {
        #[expect(clippy::match_bool)]
        match side {
            LEFT => [
                if j > 0 {
                    let top    = (i, j - 1, TOP);
                    let bottom = (i + 1, j - 1, TOP);
                    match get_cell(i, j - 1) {
                        b'/'  => [Some(bottom), None],
                        b'\\' => [Some(top), None],
                        _ => [Some(top), Some(bottom)],
                    }
                } else {[None, None]},
                if j < num_cols {
                    let top    = (i, j, TOP);
                    let bottom = (i + 1, j, TOP);
                    match get_cell(i, j) {
                        b'/'  => [Some(top), None],
                        b'\\' => [Some(bottom), None],
                        _ => [Some(top), Some(bottom)],
                    }
                } else {[None, None]}
            ],
            TOP => [
                if i > 0 {
                    let left  = (i - 1, j, LEFT);
                    let right = (i - 1, j + 1, LEFT);
                    match get_cell(i - 1, j) {
                        b'/'  => [Some(right), None],
                        b'\\' => [Some(left), None],
                        _ => [Some(left), Some(right)],
                    }
                } else {[None, None]},
                if i < num_rows {
                    let left  = (i, j, LEFT);
                    let right = (i, j + 1, LEFT);
                    match get_cell(i, j) {
                        b'/'  => [Some(left), None],
                        b'\\' => [Some(right), None],
                        _ => [Some(left), Some(right)],
                    }
                } else {[None, None]},
            ],
        }   .into_iter()
            .flatten()
            .flatten()
    }
}

mod graph {
    use std::{
        collections::HashSet,
        hash::Hash,
    };

    pub fn num_connected_components<T, V, S, F>(
        verts: V,
        succs: F) -> usize
    where
        T: Clone + Eq + Hash,
        V: Iterator<Item = T>,
        S: Iterator<Item = T>,
        F: Fn(T) -> S {
        let mut unvisited = verts.collect::<HashSet<_>>();
        let mut stack = Vec::new();
        let mut count = 0;
        while let Some(start_node) = unvisited.iter().next().cloned() {
            count += 1;
            stack.push(start_node);
            while let Some(node) = stack.pop() {
                unvisited.remove(&node);
                for succ in succs(node) {
                    if unvisited.contains(&succ) {
                        stack.push(succ);
                    }
                }
            }
        }
        count
    }
}

fn main() {
    use leetcode::input::Input;
    assert_eq!(solution::main([" /", "/ "].input()), 2);
    assert_eq!(solution::main([" /", "  "].input()), 1);
    assert_eq!(solution::main(["/\\", "\\/"].input()), 5);
}
