fn solution(arr0: Vec<i32>, arr1: Vec<i32>) -> bool {
    let mut freq0 = [0u16; 1003];
    let mut freq1 = [0u16; 1003];
    for num in arr0 { freq0[num as usize] += 1; }
    for num in arr1 { freq1[num as usize] += 1; }
    freq0 == freq1
}

fn main() {
    assert!(solution(vec![1, 2, 3, 4], vec![2, 4, 1, 3]));
    assert!(solution(vec![7], vec![7]));
    assert!(!solution(vec![3, 7, 9], vec![3, 7, 11]));
}
