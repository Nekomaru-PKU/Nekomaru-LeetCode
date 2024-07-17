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

