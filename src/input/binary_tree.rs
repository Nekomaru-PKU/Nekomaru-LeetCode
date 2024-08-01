use crate::types::binary_tree::prelude::*;

pub fn from_vec(vec: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(&root_val) = vec.first() {
        enum Child { Left, Right }

        use std::collections::VecDeque;
        let mut queue = VecDeque::new();

        let root = Rc::new(RefCell::new(TreeNode::new(root_val)));
        queue.push_back((root.clone(), Child::Left));
        queue.push_back((root.clone(), Child::Right));

        for val in vec.into_iter().skip(1) {
            if val > 0 {
                let new_node = Rc::new(RefCell::new(TreeNode::new(val)));
                queue.push_back((new_node.clone(), Child::Left));
                queue.push_back((new_node.clone(), Child::Right));

                let (node, child) = queue.pop_front().unwrap();
                let node = &mut *node.borrow_mut();
                match child {
                    Child::Left  => node.left  = Some(new_node),
                    Child::Right => node.right = Some(new_node),
                };
            } else {
                queue.pop_front().unwrap();
            }
        }

        Some(root)
    } else {
        None
    }
}
