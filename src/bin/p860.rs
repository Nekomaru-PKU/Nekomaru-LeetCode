fn solution(bills: Vec<i32>) -> bool {
    let mut bill_5 = 0;
    let mut bill_10 = 0;
    for &bill in &bills {
        match bill {
            5  => bill_5 += 1,
            10 if bill_5 >= 1 => {
                bill_10 += 1;
                bill_5  -= 1;
            },
            20 if
                bill_5  >= 1 &&
                bill_10 >= 1 => {
                bill_5  -= 1;
                bill_10 -= 1;
            },
            20 if
                bill_5 >= 3 =>
                bill_5 -= 3,
            _ => return false,
        }
    }
    true
}

fn main() {
    assert!(solution(vec![5, 5, 5, 10, 20]));
    assert!(!solution(vec![5, 5, 10, 10, 20]));
    assert!(!solution(vec![5, 5, 20, 5, 5, 10, 5, 10, 5, 20]));
    assert!(solution(vec![5, 5, 10, 20, 5, 5, 5, 5, 5, 10, 5, 5, 20, 5, 20, 5]));
}
