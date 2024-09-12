use leetcode::{
    prelude::linked_list::ListNode,
    include::linked_list,
};

fn solution(
    m: i32,
    n: i32,
    head: Option<Box<ListNode>>) -> Vec<Vec<i32>> {
    let num_rows = m;
    let num_cols = n;
    let mut mat = vec![vec![-1; num_cols as _]; num_rows as _];
    let mut row = 0;
    let mut col = 0;
    let mut row_step = 0;
    let mut col_step = 1;
    for num in linked_list::iter(&head) {
        mat[row as usize][col as usize] = num;
        if  !(0..num_rows).contains(&(row + row_step)) ||
            !(0..num_cols).contains(&(col + col_step)) ||
            mat [(row + row_step) as usize]
                [(col + col_step) as usize] != -1 {
            (row_step, col_step) = (col_step, -row_step);
        }
        row += row_step;
        col += col_step;
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
