use leetcode::{
    prelude::linked_list::ListNode,
    include::linked_list,
};

fn solution(
    m: i32,
    n: i32,
    head: Option<Box<ListNode>>) -> Vec<Vec<i32>> {
    #![expect(clippy::unreachable)]

    let num_rows = m as usize;
    let num_cols = n as usize;

    // to avoid accidental usages of `m` and `n`
    #[expect(unused_variables)] {
        let n = ();
        let m = ();
    }

    let mut mat = vec![vec![-1; num_cols]; num_rows];

    let mut row = 0;
    let mut col = 0;
    let mut row_step = 0;
    let mut col_step = 1;
    for num in linked_list::iter(&head) {
        mat[row][col] = num;
        if match (row_step, col_step) {
            ( 0,  1) => col == num_cols - 1 || mat[row][col + 1] != -1,
            ( 1,  0) => row == num_rows - 1 || mat[row + 1][col] != -1,
            ( 0, -1) => col == 0            || mat[row][col - 1] != -1,
            (-1,  0) => row == 0            || mat[row - 1][col] != -1,
            _ => unreachable!(),
        } {
            (row_step, col_step) = (col_step, -row_step);
        }
        match (row_step, col_step) {
            ( 0,  1) => col += 1,
            ( 1,  0) => row += 1,
            ( 0, -1) => col -= 1,
            (-1,  0) => row -= 1,
            _ => unreachable!(),
        }
    }
    mat
}

fn main() {
    assert_eq!(
        solution(3, 5, linked_list::from_iter([3, 0, 2, 6, 8, 1, 7, 9, 4, 2, 5, 5, 0])),
        [
            [3, 0,  2,  6, 8],
            [5, 0, -1, -1, 1],
            [5, 2,  4,  9, 7],
        ]);
    assert_eq!(
        solution(1, 4, linked_list::from_iter([0, 1, 2])),
        [[0, 1, 2, -1]]);
}
