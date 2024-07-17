mod solution {
    use std::{
        rc::Rc,
        cell::RefCell,
    };

    pub struct TreeNode {
        pub val: i32,
        pub left: Option<Rc<RefCell<TreeNode>>>,
        pub right: Option<Rc<RefCell<TreeNode>>>,
    }
    
    impl TreeNode {
        pub fn new(val: i32) -> Self {
            TreeNode {
                val,
                left: None,
                right: None
            }
        }
    }

    pub fn main(
        root: Option<Rc<RefCell<TreeNode>>>,
        src: i32,
        dst: i32,
    ) -> String {
        let root = root.unwrap();
        let root = &*root.borrow();
        let root_to_src = {
            let mut path = Vec::new();
            search_step(b'_', root, src, &mut path);
            path.reverse();
            path
        };
        let root_to_dst = {
            let mut path = Vec::new();
            search_step(b'_', root, dst, &mut path);
            path.reverse();
            path
        };
        let ancester_count = root_to_src.iter()
            .zip(root_to_dst.iter())
            .take_while(|(a, b)| a == b)
            .count();

        use std::iter;
        String::from_utf8(iter::repeat(b'U')
            .take(root_to_src.len() - ancester_count)
            .chain(root_to_dst[ancester_count..].iter().map(|c| *c))
            .collect()).unwrap()
    }

    fn search_step(
        branch: u8,
        node: &TreeNode,
        target: i32,
        target_to_root: &mut Vec<u8>,
    ) -> bool{
        if node.val == target {
            target_to_root.push(branch);
            return true;
        }
        if let Some(ref child) = node.left {
            let child = &*child.borrow();
            if search_step(b'L', child, target, target_to_root) {
                target_to_root.push(branch);
                return true;
            }
        }
        if let Some(ref child) = node.right {
            let child = &*child.borrow();
            if search_step(b'R', child, target, target_to_root) {
                target_to_root.push(branch);
                return true;
            }
        }
        false
    }
}

fn main() {
    let _ = solution::main(None, 0, 0);
}
