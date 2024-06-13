mod solution {
    pub fn main(mut seats: Vec<i32>, mut students: Vec<i32>) -> i32 {
        seats.sort_unstable();
        students.sort_unstable();
        students.iter().zip(seats.iter()).map(|(a, b)| (a - b).abs()).sum()
    }
}

fn main() {
    assert_eq!(
        solution::main(
            vec![3,1,5],
            vec![2,7,4]),
            4);
    assert_eq!(
        solution::main(
            vec![4,1,5,9],
            vec![1,3,2,6]),
            7);
    assert_eq!(
        solution::main(
            vec![2,2,6,6],
            vec![1,3,2,6],),
            4);
}
