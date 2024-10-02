fn solution(arr: Vec<i32>) -> Vec<i32> {
    use std::collections::{
        BTreeSet,
        HashMap,
    };
    let items = {
        arr
            .iter()
            .copied()
            .collect::<BTreeSet<_>>()
    };
    let ranks = {
        items
            .iter()
            .enumerate()
            .map(|(i, &v)| (v, i as i32 + 1))
            .collect::<HashMap<_, _>>()
    };
    arr.iter().map(|v| ranks[v]).collect()
}

fn main() {
    assert_eq!(solution(vec![40, 10, 20, 30]), [4, 1, 2, 3]);
    assert_eq!(solution(vec![100, 100, 100]), [1, 1, 1]);
    assert_eq!(solution(vec![37, 12, 28, 9, 100, 56, 80, 5, 12]), [5, 3, 4, 2, 8, 6, 7, 1, 3]);
}
