pub mod linked_list {
    #[derive(PartialEq, Eq, Clone, Debug)]
    pub struct ListNode {
        pub val: i32,
        pub next: Option<Box<ListNode>>,
    }

    impl ListNode {
        pub const fn new(val: i32) -> Self {
            Self { next: None, val }
        }
    }
}

pub mod binary_tree {
    pub use std::{
        cell::RefCell,
        rc::Rc,
    };

    #[derive(Debug, PartialEq, Eq)]
    pub struct TreeNode {
        pub val: i32,
        pub left: Option<Rc<RefCell<TreeNode>>>,
        pub right: Option<Rc<RefCell<TreeNode>>>,
    }

    impl TreeNode {
        pub const fn new(val: i32) -> Self {
            Self { val, left: None, right: None }
        }
    }
}
