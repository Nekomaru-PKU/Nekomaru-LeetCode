mod solution {
    use leetcode::binary_tree::prelude::*;

    pub fn main(root: Option<Rc<RefCell<TreeNode>>>, distance: i32) -> i32 {
        use std::collections::HashMap;

        type TreeNodeRc = Rc<RefCell<TreeNode>>;
        type TreeNodeOpaquePtr = *const RefCell<TreeNode>;
        type TreeNodeParentMap = HashMap<TreeNodeOpaquePtr, TreeNodeRc>;

        fn traverse_and_collect_leafs_and_parents(
            node: &TreeNodeRc,
            leafs: &mut Vec<TreeNodeRc>,
            parents: &mut TreeNodeParentMap) {
            if let &TreeNode { val: _, left: None, right: None } = &*node.borrow() {
                leafs.push(node.clone());
            } else {
                if let Some(ref child) = node.borrow().left {
                    parents.insert(Rc::as_ptr(child), node.clone());
                    traverse_and_collect_leafs_and_parents(child, leafs, parents);
                }
                if let Some(ref child) = node.borrow().right {
                    parents.insert(Rc::as_ptr(child), node.clone());
                    traverse_and_collect_leafs_and_parents(child, leafs, parents);
                }
            }
        }

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

        fn search(
            node: &TreeNodeRc,
            prev: TreeNodeOpaquePtr,
            steps: i32,
            parents: &TreeNodeParentMap) -> i32 {
            #[cfg(debug_assertions)]
            println!("search val={}, steps={}", node.borrow().val, steps);

            (if let &TreeNode { val: _, left: None, right: None } = &*node.borrow() {
                if Rc::as_ptr(node) != prev {
                    #[cfg(debug_assertions)]
                    println!("found val={}", node.borrow().val);
                    1
                } else {0}
            } else {0}) +
            (if steps >= 1 {[
                node.borrow().left.as_ref(),
                node.borrow().right.as_ref(),
                parents.get(&Rc::as_ptr(node)),
            ]
                .into_iter()
                .flatten()
                .filter(|next| Rc::as_ptr(next) != prev)
                .map(|next| search(next, Rc::as_ptr(node), steps - 1, parents))
                .sum()
            } else {0})
        }

        leafs.iter()
            .map(|leaf| search(leaf, Rc::as_ptr(leaf), distance, &parents))
            .sum::<i32>() / 2
    }
}

fn main() {
    use leetcode::binary_tree;
    assert_eq!(solution::main(binary_tree::from_vec(vec![1, 2, 3, 0, 4]), 3), 1);
    assert_eq!(solution::main(binary_tree::from_vec(vec![1, 2, 3, 4, 5, 6, 7]), 3), 2);
    assert_eq!(solution::main(binary_tree::from_vec(vec![7, 1, 4, 6, 0, 5, 3, 0, 0, 0, 0, 0, 2]), 3), 1);
}
