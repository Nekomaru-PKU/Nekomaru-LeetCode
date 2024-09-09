use leetcode::{
    prelude::linked_list::ListNode,
    include::linked_list,
};

fn solution(head: Option<Box<ListNode>>, k: i32) -> Vec<Option<Box<ListNode>>> {
    let data = linked_list::iter(&head).collect::<Vec<_>>();
    let num_parts = k as usize;
    let shorter_part_len = data.len() / num_parts;
    let longer_part_len = shorter_part_len + 1;
    let num_longer_parts = data.len() - shorter_part_len * num_parts;
    let num_shorter_parts = num_parts - num_longer_parts;
    Iterator::chain(
        (0..num_longer_parts)
            .map(|i| linked_list::from_iter({
                data[longer_part_len * i..]
                    [..longer_part_len]
                    .iter()
                    .copied()
            })),
        (0..num_shorter_parts)
            .map(|i| linked_list::from_iter({
                data[longer_part_len * num_longer_parts..]
                    [shorter_part_len * i..]
                    [..shorter_part_len]
                    .iter()
                    .copied()
        })))
        .collect()
}


fn main() {
    fn to_output(data: &[&[i32]]) -> Vec<Option<Box<ListNode>>> {
        data.into_iter()
            .map(|arr| arr.iter().copied())
            .map(linked_list::from_iter)
            .collect::<Vec<_>>()
    }

    use std::iter;
    assert_eq!(
        solution(linked_list::from_iter(iter::empty()), 2),
        to_output(&[&[], &[]]));
    assert_eq!(
        solution(linked_list::from_iter(1..=3), 5),
        to_output(&[&[1], &[2], &[3], &[], &[]]));
    assert_eq!(
        solution(linked_list::from_iter(1..=10), 3),
        to_output(&[&[1, 2, 3, 4], &[5, 6, 7], &[8, 9, 10]]));
}
