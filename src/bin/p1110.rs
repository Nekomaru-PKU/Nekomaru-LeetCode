mod solution {
    use leetcode::binary_tree::*;

    use std::collections::HashMap;

    pub fn main(
        root: Option<Rc<RefCell<TreeNode>>>,
        to_delete: Vec<i32>,
    ) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        use std::iter;

        let ptr_map = gather_ptr_map(&root);
        let parent_map = gather_parent_map(&root);

        let mut has_parent = ptr_map.keys()
            .cloned()
            .zip(iter::repeat(true))
            .collect::<HashMap<_, _>>();
        *has_parent.get_mut(&root.unwrap().borrow().val).unwrap() = false;

        for val in to_delete {
            if let Some(parent_val) = parent_map.get(&val) {
                let parent: &mut TreeNode = &mut ptr_map[&parent_val].borrow_mut();
                if let Some(ref child) = parent.left {
                    if child.borrow().val == val {
                        parent.left = None;
                    }
                }
                if let Some(ref child) = parent.right {
                    if child.borrow().val == val {
                        parent.right = None;
                    }
                }
            }

            let node = &*ptr_map[&val].borrow();
            has_parent.remove(&val);
            if let Some(ref child) = node.left {
                *has_parent.get_mut(&child.borrow().val).unwrap() = false;
            }
            if let Some(ref child) = node.right {
                *has_parent.get_mut(&child.borrow().val).unwrap() = false;
            }
        }

        has_parent.into_iter()
            .filter(|(_, has_parent)| !has_parent)
            .map(|(val, _)| Some(ptr_map[&val].clone()))
            .collect()
    }

    fn gather_ptr_map(root: &Option<Rc<RefCell<TreeNode>>>) -> HashMap<i32, Rc<RefCell<TreeNode>>> {
        fn visit(
            node: &Rc<RefCell<TreeNode>>,
            map: &mut HashMap<i32, Rc<RefCell<TreeNode>>>) {
            let node_cloned = node.clone();
            let node = &*node.borrow();
            map.insert(node.val, node_cloned);
            if let Some(ref node) = node.left {
                visit(node, map);
            }
            if let Some(ref node) = node.right {
                visit(node, map);
            }
        }

        let mut map = HashMap::new();
        if let Some(root) = root {
            visit(&root, &mut map);
        }
        map
    }

    fn gather_parent_map(root: &Option<Rc<RefCell<TreeNode>>>) -> HashMap<i32, i32> {
        fn visit(
            node: &Rc<RefCell<TreeNode>>,
            map: &mut HashMap<i32, i32>) {
            let node = &*node.borrow();
            if let Some(ref child) = node.left {
                map.insert(child.borrow().val, node.val);
                visit(child, map);
            }
            if let Some(ref child) = node.right {
                map.insert(child.borrow().val, node.val);
                visit(child, map);
            }
        }

        let mut map = HashMap::new();
        if let Some(root) = root {
            visit(&root, &mut map);
        }
        map
    }
}

fn main() {
    use leetcode::binary_tree::*;
    let _ = solution::main(Some(Rc::new(RefCell::new(TreeNode { val: 0, left: Some(Rc::new(RefCell::new(TreeNode { val: 1, left: None, right: None }))), right: None }))), vec![0]);
}
