use leetcode::prelude::linked_list::ListNode;

fn solution(
    nums: Vec<i32>,
    mut head: Option<Box<ListNode>>)
-> Option<Box<ListNode>> {
    use std::{
        collections::HashSet,
        mem,
    };
    let nums = nums
        .iter()
        .copied()
        .collect::<HashSet<_>>();
    let mut node = &mut head;
    while node.is_some() {
        if nums.contains(&node.as_ref().unwrap().val) {
            *node = mem::take(node).unwrap().next;
        } else {
            node = &mut node.as_mut().unwrap().next;
        }
    }
    head
}

fn main() {
    use leetcode::include::linked_list;
    assert_eq!(solution(
        vec![1, 2, 3],
        linked_list::from_iter([1, 2, 3, 4, 5])),
        linked_list::from_iter([4, 5]));
    assert_eq!(solution(
        vec![1],
        linked_list::from_iter([1, 2, 1, 2, 1, 2])),
        linked_list::from_iter([2, 2, 2]));
    assert_eq!(solution(
        vec![5],
        linked_list::from_iter([1, 2, 3, 4])),
        linked_list::from_iter([1, 2, 3, 4]));
}
