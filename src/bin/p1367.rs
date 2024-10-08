mod solution {
    use leetcode::prelude::{
        binary_tree::*,
        linked_list::ListNode,
    };

    pub fn main(
        head: Option<Box<ListNode>>,
        root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let head = *head.unwrap();
        let root =  root.unwrap();
        let mut start_nodes = Vec::new();
        find_node_by_val(
            &root,
            head.val,
            &mut start_nodes);
        start_nodes
            .iter()
            .any(|node| is_match(&head, node))
    }

    /* this is invalid due to overflow evaluating some type requirement*:
    fn find_node_by_val(
        root_rc: &Rc<RefCell<TreeNode>>,
        val: i32)
     -> impl Iterator<Item = Rc<RefCell<TreeNode>>> {
        let root = &*root_rc.borrow();
        (root.val == val)
            .then_some(Rc::clone(root_rc))
            .into_iter()
            .chain([
                root.left .as_ref(),
                root.right.as_ref(),
            ]   .into_iter()
                .flatten()
                .map(|child| find_node_by_val(child, val))
                .flatten())
    }*/

    fn find_node_by_val(
        root_rc: &Rc<RefCell<TreeNode>>,
        val: i32,
        out: &mut Vec<Rc<RefCell<TreeNode>>>) {
        let root = &*root_rc.borrow();
        out.extend((root.val == val).then_some(Rc::clone(root_rc)));
        for child in [
            root.left .as_ref(),
            root.right.as_ref(),
        ].into_iter().flatten() {
            find_node_by_val(child, val, out);
        }
    }

    fn is_match(
        head: &ListNode,
        root: &Rc<RefCell<TreeNode>>) -> bool {
        let root = root.borrow();
        head.val == root.val &&
        head.next
            .as_ref()
            .map_or(true, |next| [
                root.left .as_ref(),
                root.right.as_ref(),
            ]   .into_iter()
                .flatten()
                .any(|child| is_match(next, child)))
    }
}

fn main() {
    #![allow(clippy::redundant_clone)]

    use leetcode::{
        input::binary_tree::{self, NULL},
        include::linked_list,
    };

    let tree = binary_tree::from_vec(vec![
        1, 4, 4, NULL, 2, 2, NULL, 1, NULL,
        6, 8, NULL, NULL, NULL, NULL, 1, 3,
    ]);

    assert!( solution::main(
        linked_list::from_iter([4, 2, 8]),
        tree.clone()));
    assert!( solution::main(
        linked_list::from_iter([1, 4, 2, 6]),
        tree.clone()));
    assert!(!solution::main(
        linked_list::from_iter([1, 4, 2, 6, 8]),
        tree.clone()));
}
