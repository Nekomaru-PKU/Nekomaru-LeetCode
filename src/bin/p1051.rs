fn solution(vec: Vec<i32>) -> i32 {
    let mut vec_sorted = vec.clone();
    vec_sorted.sort_unstable();
    vec .iter()
        .zip(&vec_sorted)
        .map(|(&i, &j)| if i != j {1} else {0})
        .sum()
}

fn main() {
    assert_eq!(solution(vec![1,1,4,2,1,3]), 3);
    assert_eq!(solution(vec![5,1,2,3,4]), 5);
    assert_eq!(solution(vec![1,2,3,4,5]), 0);
}
