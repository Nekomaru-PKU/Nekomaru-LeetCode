mod solution {
    pub fn main(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
        let mut zipped = names.into_iter().zip(heights).collect::<Vec<_>>();
        zipped.sort_by_key(|&(_, height)| -height);
        zipped.into_iter().map(|(name, _)| name).collect()
    }
}

fn main() {
    let _ = solution::main(Vec::new(), Vec::new());
}
