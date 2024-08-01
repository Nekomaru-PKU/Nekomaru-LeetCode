pub trait Input<T> { fn input(self) -> T; }

impl Input<i32>    for  i32 { fn input(self) -> i32    { self }}
impl Input<String> for &str { fn input(self) -> String { self.to_string() } }

impl<T, U: Input<T>, const N: usize> Input<Vec<T>> for [U; N] {
    fn input(self) -> Vec<T> {
        self.into_iter().map(Input::input).collect()
    }
}

pub mod binary_tree;
