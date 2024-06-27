mod solution {
    pub fn main(edges: Vec<Vec<i32>>) -> i32 {
        let e0v0 = edges[0][0];
        let e0v1 = edges[0][1];
        let e1v0 = edges[1][0];
        let e1v1 = edges[1][1];
        if e0v0 == e1v0 || e0v0 == e1v1 {
            e0v0
        } else {
            e0v1
        }
    }
}

fn main() {
    assert_eq!(solution::main(vec![vec![1, 2], vec![2, 3], vec![4, 2]]), 2);
    assert_eq!(solution::main(vec![vec![1, 2], vec![5, 1], vec![1, 3], vec![1, 4]]), 1);
}
