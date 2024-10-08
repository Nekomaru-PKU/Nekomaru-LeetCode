use leetcode::prelude::binary_tree::*;

fn solution(
    root: Option<Rc<RefCell<TreeNode>>>,
    to_delete: Vec<i32>,
) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    use std::collections::HashMap;
    use std::iter;

    type TreeNodeRc = Rc<RefCell<TreeNode>>;
    type TreeNodeOpaquePtr = *const RefCell<TreeNode>;

    fn traverse_and_collect_nodes_and_parents(
        node: &TreeNodeRc,
        nodes: &mut HashMap<i32, TreeNodeRc>,
        parents: &mut HashMap<TreeNodeOpaquePtr, TreeNodeRc>) {
        let TreeNode { val, ref left, ref right } = *node.borrow();
        nodes.insert(val, Rc::clone(node));
        [left, right]
            .into_iter()
            .flat_map(|child| child.as_ref())
            .for_each(|child| {
                parents.insert(Rc::as_ptr(child), Rc::clone(node));
                traverse_and_collect_nodes_and_parents(child, nodes, parents);
            });
    }

    let (nodes, parents) = {
        let mut nodes = HashMap::new();
        let mut parents = HashMap::new();
        traverse_and_collect_nodes_and_parents(
            root.as_ref().unwrap(),
            &mut nodes,
            &mut parents);
        (nodes, parents)
    };

    let mut has_parent = nodes
        .values()
        .map(Rc::as_ptr)
        .zip(iter::repeat(true))
        .collect::<HashMap<_, _>>();
    has_parent
        .get_mut(&Rc::as_ptr(root.as_ref().unwrap()))
        .map(|val| *val = false)
        .unwrap();

    for val in to_delete {
        let node = &nodes[&val];
        let node_ptr = Rc::as_ptr(node);

        // update tree structure
        if let Some(parent) = parents.get(&node_ptr) {
            let TreeNode {
                ref mut left,
                ref mut right,
                ..
            } = *parent.borrow_mut();
            [left, right]
                .into_iter()
                .filter(|child| child
                    .as_ref()
                    .is_some_and(|child| child.borrow().val == val))
                .for_each(|child| *child = None);
        }

        // update `has_parent`
        has_parent.remove(&node_ptr); {
            let TreeNode { ref left, ref right, .. } = *node.borrow();
            [left, right]
                .into_iter()
                .flat_map(|child| child.as_ref())
                .for_each(|child| has_parent
                    .get_mut(&Rc::as_ptr(child))
                    .map(|val| *val = false)
                    .unwrap());
        }
    }

    nodes.values()
        .filter(|&node| has_parent
            .get(&Rc::as_ptr(node))
            .copied()
            .is_some_and(|has_parent| !has_parent))
        .cloned()
        .map(Option::Some)
        .collect()
}

fn main() {
    use leetcode::{
        cmp,
        input::binary_tree::{self, NULL},
    };
    assert!(cmp::eq_any_order(solution(
        binary_tree::from_vec(vec![1, 2, 3, 4, 5, 6, 7]), vec![3, 5]), [
        binary_tree::from_vec(vec![1, 2, NULL, 4]),
        binary_tree::from_vec(vec![6]),
        binary_tree::from_vec(vec![7]),
    ]));
    assert!(cmp::eq_any_order(solution(
        binary_tree::from_vec(vec![1, 2, 4, NULL, 3]), vec![3]), [
        binary_tree::from_vec(vec![1, 2, 4]),
    ]));
}
