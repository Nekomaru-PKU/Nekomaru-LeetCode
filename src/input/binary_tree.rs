use crate::prelude::binary_tree::*;

pub const NULL: i32 = i32::MIN;

#[inline]
pub fn from_vec(vec: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(&root_val) = vec.first() {
        enum Child { Left, Right }

        use std::collections::VecDeque;
        let mut queue = VecDeque::new();

        let root = Rc::new(RefCell::new(TreeNode::new(root_val)));
        queue.push_back((Rc::clone(&root), Child::Left));
        queue.push_back((Rc::clone(&root), Child::Right));

        for val in vec.into_iter().skip(1) {
            if val != NULL {
                let new_node = Rc::new(RefCell::new(TreeNode::new(val)));
                queue.push_back((Rc::clone(&new_node), Child::Left));
                queue.push_back((Rc::clone(&new_node), Child::Right));

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
