use leetcode::{
    prelude::linked_list::ListNode,
    include::linked_list,
};

fn solution(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    // we skip the preceeding zero node.
    let mut curr = &head.unwrap().next;

    let mut output = None;
    let mut output_tail = &mut output;
    let mut sum = 0;
    while let &Some(ref node) = curr {
        let &ListNode { val, ref next } = &**node;
        if val == 0 {
            *output_tail = Some(Box::new(ListNode::new(sum)));
            output_tail = &mut output_tail.as_mut().unwrap().next;
            sum = 0;
        } else {
            sum += val;
        }
        curr = next;
    }
    output
}

fn main() {
    assert_eq!(solution(
        linked_list::from_iter([0, 3, 1, 0, 4, 5, 2, 0])),
        linked_list::from_iter([4, 11]));
    assert_eq!(solution(
        linked_list::from_iter([0, 1, 0, 3, 0, 2, 2, 0])),
        linked_list::from_iter([1, 3, 4]));
}
