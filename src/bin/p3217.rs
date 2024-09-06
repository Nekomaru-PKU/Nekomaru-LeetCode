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
    assert_eq!(solution(vec![], None), None);
}
