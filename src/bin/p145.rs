use leetcode::prelude::binary_tree::*;

fn solution(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let Some(root) = root else { return Vec::new(); };
    let mut out = Vec::new();

    struct State {
        node: Rc<RefCell<TreeNode>>,
        expanded: bool,
        next_sibling: Option<Rc<RefCell<TreeNode>>>
    }

    let mut stack = vec![State {
        node: root,
        expanded: false,
        next_sibling: None,
    }];

    while let Some(State {
        node,
        expanded,
        next_sibling,
    }) = stack.pop() {
        if !expanded {
            stack.push(State {
                node: Rc::clone(&node),
                expanded: true,
                next_sibling,
            });
            match *node.borrow() {
                TreeNode {
                    left: Some(ref child),
                    ref right,
                    ..
                } => stack.push(State {
                    node: Rc::clone(child),
                    expanded: false,
                    next_sibling: right.clone(),
                }),
                TreeNode {
                    left: None,
                    right: Some(ref child),
                    ..
                } => stack.push(State {
                    node: Rc::clone(child),
                    expanded: false,
                    next_sibling: None,
                }),
                _ => ()
            }
        } else {
            out.push(node.borrow().val);
            if let Some(next_sibling) = next_sibling {
                stack.push(State {
                    node: next_sibling,
                    expanded: false,
                    next_sibling: None,
                });
            }
        }
    }

    out
}

fn main() {
    use leetcode::input::binary_tree::{self, NULL};
    assert_eq!(solution(binary_tree::from_vec(vec![1, NULL, 2, 3])), [3, 2, 1]);
    assert_eq!(solution(binary_tree::from_vec(vec![1])), [1]);
    assert_eq!(solution(binary_tree::from_vec(vec![])), []);
}
