pub mod prelude {
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
}

pub mod include {
    pub mod linked_list;
}

pub mod input {
    pub mod binary_tree;

    pub trait Input<T> { fn input(self) -> T; }

    impl Input<Self>   for  i32 { #[inline] fn input(self) -> Self   { self }}
    impl Input<String> for &str { #[inline] fn input(self) -> String { self.to_owned() } }

    impl<T, U: Input<T>, const N: usize> Input<Vec<T>> for [U; N] {
        #[inline]
        fn input(self) -> Vec<T> {
            self.into_iter().map(Input::input).collect()
        }
    }
}

pub mod cmp {
    #[inline]
    pub fn eq_any_order<T: PartialEq>(v1: &[T], v2: &[T]) -> bool {
        v1.iter().all(|item| v2.contains(item)) &&
        v2.iter().all(|item| v1.contains(item))
    }
}

pub mod perf {
    #[inline]
    pub fn time<T, F: FnOnce() -> T>(name: &str, inner_fn: F) -> T {
        use std::time::Instant;
        let start = Instant::now();
        let out = inner_fn();
        let elapsed_ms = start.elapsed().as_secs_f32() * 1000.0;
        println!("test case '{name}' succeeded in {elapsed_ms}ms");
        out
    }
}
