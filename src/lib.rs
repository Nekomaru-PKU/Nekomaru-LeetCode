pub mod prelude {
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
}

pub mod include {
    pub mod iter_ext;
    pub mod linked_list;
    pub mod math;
}

pub mod input {
    pub mod binary_tree;

    pub trait Input<T> { fn input(self) -> T; }

    impl Input<Self>   for  i32 { #[inline] fn input(self) -> Self   { self }}
    impl Input<String> for &str { #[inline] fn input(self) -> String { self.to_owned() } }

    impl<T, U: Input<T>, const N: usize> Input<Vec<T>> for [U; N] {
        fn input(self) -> Vec<T> {
            self.into_iter().map(Input::input).collect()
        }
    }
}

pub mod cmp {
    pub fn eq_any_order<T1, T2, I1, I2>(i1: I1, i2: I2) -> bool
    where
        T1: PartialEq<T2>,
        T2: PartialEq<T1>,
        I1: Clone + IntoIterator<Item = T1>,
        I2: Clone + IntoIterator<Item = T2>, {
        i1.clone().into_iter().all(|v1| {
            i2.clone().into_iter().find(|v2| v1.eq(v2)).is_some()
        }) &&
        i2.clone().into_iter().all(|v2| {
            i1.clone().into_iter().find(|v1| v2.eq(v1)).is_some()
        })
    }
}

pub mod perf {
    pub fn time<T, F: FnOnce() -> T>(name: &str, inner_fn: F) -> T {
        use std::time::Instant;
        let start = Instant::now();
        let out = inner_fn();
        let elapsed_ms = start.elapsed().as_secs_f32() * 1000.0;
        println!("test case '{name}' succeeded in {elapsed_ms}ms");
        out
    }
}
