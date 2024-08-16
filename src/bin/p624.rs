fn solution(arrays: Vec<Vec<i32>>) -> i32 {
    let mut min = (i32::MAX, usize::MAX);
    let mut max = (i32::MIN, usize::MAX);
    let mut second_min = (i32::MAX, usize::MAX);
    let mut second_max = (i32::MIN, usize::MAX);
    for (i, arr) in arrays.iter().enumerate() {
        for num in [arr[0], arr[arr.len() - 1]] {
            if num < min.0 {
                (min, second_min) = ((num, i), min);
            } else if num < second_min.0 {
                second_min = (num, i);
            }
            if num > max.0 {
                (max, second_max) = ((num, i), max);
            } else if num > second_max.0 {
                second_max = (num, i);
            }
        }
    }

    if min.1 != max.1 {
        max.0 - min.0
    } else {
        i32::max(
            second_max.0 - min.0,
            max.0 - second_min.0)
    }
}

fn main() {
    assert_eq!(solution(vec![
        vec![1, 2, 3],
        vec![4, 5],
        vec![1, 2, 3]]), 4);
    assert_eq!(solution(vec![vec![1], vec![1]]), 0);
}
