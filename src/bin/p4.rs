fn solution_core(
    mut arr0: &[i32],
    mut arr1: &[i32],
    mut k: usize) -> i32 {
    #![expect(clippy::missing_asserts_for_indexing)]
    loop {
        debug_assert!(!(arr0.is_empty() && arr1.is_empty()));
        debug_assert!(k < arr0.len() + arr1.len());

        if arr0.is_empty() { return arr1[k]; }
        if arr1.is_empty() { return arr0[k]; }
        if arr0.len() == 1 && arr1.len() == 1 {
            return if k == 0 {
                arr0[0].min(arr1[0])
            } else {
                arr0[0].max(arr1[0])
            }
        }

        let m0i0 = arr0.len() / 2;
        let m1i1 = arr1.len() / 2;
        let m0 = arr0[m0i0];
        let m1 = arr1[m1i1];
        let m0i1 = arr1.binary_search(&m0).unwrap_or_else(|i| i);
        let m1i0 = arr0.binary_search(&m1).unwrap_or_else(|i| i);

        #[expect(clippy::collapsible_else_if)]
        if m0 <= m1 {
            if k < m0i0 + m0i1 {
                arr0 = &arr0[..m0i0];
                arr1 = &arr1[..m0i1];
            } else if k < m1i0 + m1i1 {
                arr0 = &arr0[m0i0..m1i0];
                arr1 = &arr1[m0i1..m1i1];
                k -= m0i0 + m0i1;
            } else {
                arr0 = &arr0[m1i0..];
                arr1 = &arr1[m1i1..];
                k -= m1i0 + m1i1;
            }
        } else {
            if k < m1i0 + m1i1 {
                arr0 = &arr0[..m1i0];
                arr1 = &arr1[..m1i1];
            } else if k < m0i0 + m0i1 {
                
                arr0 = &arr0[m1i0..m0i0];
                arr1 = &arr1[m1i1..m0i1];
                k -= m1i0 + m1i1;
            } else {
                arr0 = &arr0[m0i0..];
                arr1 = &arr1[m0i1..];
                k -= m0i0 + m0i1;
            };
        }
    }
}

fn solution(arr0: Vec<i32>, arr1: Vec<i32>) -> f64 {
    if (arr0.len() + arr1.len()) % 2 == 1 {
        solution_core(
            &arr0,
            &arr1,
            (arr0.len() + arr1.len()) / 2) as _
    } else {
        (solution_core(
            &arr0,
            &arr1,
            (arr0.len() + arr1.len()) / 2 - 1) +
        solution_core(
            &arr0,
            &arr1,
            (arr0.len() + arr1.len()) / 2)) as f64 / 2.0
    }
}

fn main() {
    #![expect(clippy::float_cmp)]
    assert_eq!(solution(vec![1], vec![1]), 1.0);
    assert_eq!(solution(vec![1], vec![2]), 1.5);
    assert_eq!(solution(vec![1, 3], vec![2]), 2.0);
    assert_eq!(solution(vec![1, 2], vec![3, 4]), 2.5);
}
