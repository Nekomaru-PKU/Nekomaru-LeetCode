mod solution {
    pub fn main(arr0: Vec<i32>, arr1: Vec<i32>) -> Vec<i32> {
        let mut count = [0; 1003];
        for &n in &arr0 {
            count[n as usize] += 1;
        }

        let mut out = Vec::new();
        for &n in &arr1 {
            if count.get(n as usize).copied().unwrap_or(0) > 0 {
                out.push(n);
                count[n as usize] -= 1;
            }
        }
        out
    }
}

fn main() {
    assert_eq!(solution::main(vec![1, 2, 2, 1], vec![2, 2]), vec![2, 2]);
    assert_eq!(solution::main(vec![4, 9, 5], vec![9, 4, 9, 8, 4]), vec![9, 4]);
}
