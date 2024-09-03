pub mod prelude;
pub mod input;

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
