fn solution(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let row_min = matrix.iter()
        .map(|row| row.iter()
            .cloned()
            .min()
            .unwrap())
        .collect::<Vec<_>>();
    let column_max = matrix.first()
        .unwrap()
        .iter()
        .enumerate()
        .map(|(j, _)| matrix.iter()
            .map(|row| row[j])
            .max()
            .unwrap())
        .collect::<Vec<_>>();
    matrix.iter()
        .zip(row_min.iter())
        .flat_map(|(row, &row_min)| row.iter()
            .zip(column_max.iter())
            .filter(move |&(&val, &column_max)|
                val == row_min &&
                val == column_max)
            .map(|(&val, _)| val))
        .collect()
}

fn main() {
    assert_eq!(
        solution(vec![
            vec![3, 7, 8],
            vec![9, 11, 13],
            vec![15, 16, 17]]),
        vec![15]);
}
