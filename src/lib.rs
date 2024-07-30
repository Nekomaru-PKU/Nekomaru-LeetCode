pub trait Input<T> { fn input(self) -> T; }

impl Input<i32>    for  i32 { fn input(self) -> i32    { self }}
impl Input<String> for &str { fn input(self) -> String { self.to_string() } }

impl<T, U: Input<T>, const N: usize> Input<Vec<T>> for [U; N] {
    fn input(self) -> Vec<T> {
        self.into_iter().map(Input::input).collect()
    }
}

pub mod binary_tree;

pub mod vec {
    pub fn eq_any_order<T: PartialEq>(v1: Vec<T>, v2: Vec<T>) -> bool {
        v1.iter().all(|item| v2.contains(item)) &&
        v2.iter().all(|item| v1.contains(item))
    }
}

pub fn print_time<T, F: FnOnce() -> T>(name: &str, inner_fn: F) -> T {
    use std::time::Instant;
    let s = Instant::now();
    let o = inner_fn();
    let t = Instant::now() - s;
    println!("test case '{name}' succeeded in {}ms", t.as_secs_f32() * 1000.0);
    o
}

