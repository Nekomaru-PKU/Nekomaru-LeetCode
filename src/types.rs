pub mod linked_list {
    #[derive(PartialEq, Eq, Clone, Debug)]
    pub struct ListNode {
        pub val: i32,
        pub next: Option<Box<ListNode>>
    }

    impl ListNode {
        pub fn new(val: i32) -> Self { ListNode { next: None, val } }
    }
}

pub mod binary_tree {
    pub mod prelude {
        pub use std::rc::Rc;
        pub use std::cell::RefCell;
        pub use super::TreeNode;
    }

    use prelude::*;

    #[derive(Debug, PartialEq, Eq)]
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
}
