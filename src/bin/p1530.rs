mod solution {
    use std::collections::HashMap;

    use leetcode::prelude::binary_tree::*;

    type TreeNodeRc = Rc<RefCell<TreeNode>>;
    type TreeNodeOpaquePtr = *const RefCell<TreeNode>;
    type TreeNodeParentMap = HashMap<TreeNodeOpaquePtr, TreeNodeRc>;

    pub fn main(root: Option<Rc<RefCell<TreeNode>>>, distance: i32) -> i32 {
        let (leafs, parents) = {
            let root = root.unwrap();
            let mut leafs = Vec::new();
            let mut parents = HashMap::new();
            traverse_and_collect_leafs_and_parents(
                &root,
                &mut leafs,
                &mut parents);
            (leafs, parents)
        };

        leafs.iter()
            .map(|leaf| search(leaf, Rc::as_ptr(leaf), distance, &parents))
            .sum::<i32>() / 2
    }

    fn traverse_and_collect_leafs_and_parents(
        node: &TreeNodeRc,
        leafs: &mut Vec<TreeNodeRc>,
        parents: &mut TreeNodeParentMap) {
        match *node.borrow() {
            TreeNode { val: _, left: None, right: None } =>
                leafs.push(node.clone()),
            TreeNode { val: _, ref left, ref right } =>
                [left, right]
                    .into_iter()
                    .flat_map(|child| child.as_ref())
                    .for_each(|child| {
                        parents.insert(Rc::as_ptr(child), node.clone());
                        traverse_and_collect_leafs_and_parents(child, leafs, parents);
                    }),
        }
    }

    fn search(
        node: &TreeNodeRc,
        prev: TreeNodeOpaquePtr,
        steps: i32,
        parents: &TreeNodeParentMap) -> i32 {
        let TreeNode { val: _, ref left, ref right } = *node.borrow();
        let node_ptr = Rc::as_ptr(node);
        (if node_ptr != prev && left.is_none() && right.is_none() {1} else {0}) +
        (if steps >= 1 {
            [left, right]
                .into_iter()
                .flat_map(|child| child.as_ref())
                .chain(parents.get(&node_ptr))
                .filter(|next| Rc::as_ptr(next) != prev)
                .map(|next| search(next, node_ptr, steps - 1, parents))
                .sum()
        } else {0})
    }
}

fn main() {
    use leetcode::input::binary_tree::{self, NULL};
    assert_eq!(solution::main(binary_tree::from_vec(vec![1, 2, 3, NULL, 4]), 3), 1);
    assert_eq!(solution::main(binary_tree::from_vec(vec![1, 2, 3, 4, 5, 6, 7]), 3), 2);
    assert_eq!(solution::main(binary_tree::from_vec(vec![7, 1, 4, 6, NULL, 5, 3, NULL, NULL, NULL, NULL, NULL, 2]), 3), 1);
}
