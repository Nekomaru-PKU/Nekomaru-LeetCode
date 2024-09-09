use crate::prelude::linked_list::ListNode;

#[derive(Clone)]
pub struct Iter<'a> {
    node: &'a Option<Box<ListNode>>,
}

impl Iterator for Iter<'_> {
    type Item = i32;

    #[inline]
    fn next(&mut self) -> Option<i32> {
        self.node
            .as_ref()
            .map(Box::as_ref)
            .map(|&ListNode { val, ref next }| {
                self.node = next;
                val
            })
    }
}

#[inline]
pub const fn iter(head: &Option<Box<ListNode>>) -> Iter<'_> {
    Iter { node: head }
}

#[inline]
pub fn from_iter<T>(iter: T) -> Option<Box<ListNode>>
where
    T: IntoIterator<Item = i32>, {
    let mut head = None;
    let mut node = &mut head;
    for val in iter {
        node.replace(Box::new(ListNode::new(val)));
        node = &mut node.as_mut().unwrap().next;
    }
    head
}
