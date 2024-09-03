fn solution(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
    // `count[item]` is the count of `item` in `arr1`.
    let mut count = [0; 1003];
    for &item in &arr1 {
        count[item as usize] += 1;
    }

    // we mark visited item with `-1` since we have the constraint
    // `0 <= arr1[i], arr2[i] <= 1000`.
    const VISITED: i32 = -1;
    let mut output = Vec::with_capacity(arr1.len());

    for &item in &arr2 {
        for _ in 0..count[item as usize] {
            output.push(item);
        }
        count[item as usize] = VISITED;
    }

    for (i, &count) in count.iter().enumerate() {
        for _ in 0..count {
            output.push(i as i32);
        }
    }

    output
}

fn main() {
    let arr1 = vec![2,3,1,3,2,4,6,7,9,2,19];
    let arr2 = vec![2,1,4,3,9,6];
    let expected = vec![2,2,2,1,4,3,3,9,6,7,19];
    assert_eq!(solution(arr1, arr2), expected);

    let arr1 = vec![28,6,22,8,44,17];
    let arr2 = vec![22,28,8,6];
    let expected = vec![22,28,8,6,17,44];
    assert_eq!(solution(arr1, arr2), expected);
}
