pub mod linked_list {
    #[expect(clippy::exhaustive_structs)]
    #[derive(PartialEq, Eq, Clone, Debug)]
    pub struct ListNode {
        pub val: i32,
        pub next: Option<Box<ListNode>>,
    }

    impl ListNode {
        #[inline]
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

    #[expect(clippy::exhaustive_structs)]
    #[derive(Debug, PartialEq, Eq)]
    pub struct TreeNode {
        pub val: i32,
        pub left: Option<Rc<RefCell<TreeNode>>>,
        pub right: Option<Rc<RefCell<TreeNode>>>,
    }

    impl TreeNode {
        #[inline]
        pub const fn new(val: i32) -> Self {
            Self { val, left: None, right: None }
        }
    }
}
