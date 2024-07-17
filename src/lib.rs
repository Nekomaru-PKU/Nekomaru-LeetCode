pub fn print_time<T, F: FnOnce() -> T>(name: &str, inner_fn: F) -> T {
    use std::time::Instant;
    let s = Instant::now();
    let o = inner_fn();
    let t = Instant::now() - s;
    println!("test case '{name}' succeeded in {}ms", t.as_secs_f32() * 1000.0);
    o
}

pub mod binary_tree {
    pub use std::{
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
}
