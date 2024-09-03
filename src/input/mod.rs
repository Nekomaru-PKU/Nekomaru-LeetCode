pub mod binary_tree;

pub trait Input<T> { fn input(self) -> T; }

impl Input<Self>   for  i32 { fn input(self) -> Self   { self }}
impl Input<String> for &str { fn input(self) -> String { self.to_owned() } }

impl<T, U: Input<T>, const N: usize> Input<Vec<T>> for [U; N] {
    fn input(self) -> Vec<T> {
        self.into_iter().map(Input::input).collect()
    }
}
