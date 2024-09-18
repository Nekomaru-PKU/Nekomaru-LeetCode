use leetcode::{
    prelude::linked_list::ListNode,
    include::linked_list,
};

mod math {
    use core::ops;

    pub fn gcd<T>(mut a: T, mut b: T) -> T
    where
        T: Default + Copy + Ord + ops::Rem<Output = T> {
        let zero = T::default();
        debug_assert!(a > zero);
        debug_assert!(b > zero);
        if    a < b    { (a, b) = (b, a); }
        while b > zero { (a, b) = (b, a % b); }
        a
    }
}

fn solution(mut head: Option<Box<ListNode>>)
 -> Option<Box<ListNode>> {
    let mut lnode = head.as_mut().unwrap().as_mut();
    while lnode.next.is_some() {
        let rnode = lnode.next.take().unwrap();
        lnode.next = Some(Box::new(ListNode {
            val: math::gcd(
                lnode.val,
                rnode.val),
            next: Some(rnode),
        }));
        lnode = lnode
            .next.as_mut().unwrap().as_mut()
            .next.as_mut().unwrap().as_mut();
    }
    head
}

fn main() {
    assert_eq!(solution(
        linked_list::from_iter([18, 6, 10, 3])),
        linked_list::from_iter([18, 6, 6, 2, 10, 1, 3]));
    assert_eq!(solution(
        linked_list::from_iter([7])),
        linked_list::from_iter([7]));
}
