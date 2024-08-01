use leetcode::types::linked_list::ListNode;

fn solution(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>)
 -> Option<Box<ListNode>> {
    if l1.as_ref().unwrap().as_ref() == &ListNode::new(0) { return l2; }
    if l2.as_ref().unwrap().as_ref() == &ListNode::new(0) { return l1; }

    let mut l1 = l1;

    let mut p1 = &mut l1;
    let mut p2 = &l2;
    let mut carry = 0;
    while p1.is_some() || p2.is_some() || carry > 0 {
        let sum =
            p1.as_ref().map(|node| node.val).unwrap_or(0) +
            p2.as_ref().map(|node| node.val).unwrap_or(0) +
            carry;

        if p1.is_none() {
            *p1 = Some(Box::new(ListNode::new(0)));
        }
        p1.as_mut().unwrap().val = sum % 10;
        carry = sum / 10;

        p1 = &mut p1.as_mut().unwrap().next;
        if p2.is_some() {
            p2 = &p2.as_ref().unwrap().next;
        }
    }

    l1
}

fn main() { let _ = solution(None, None); }
