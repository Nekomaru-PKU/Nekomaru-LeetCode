mod solution {
    pub fn main(expr: String) -> Vec<i32> {
        let (operands, ops) = parse_expr(&expr);
        let num_operands = operands.len();
        let mut dp = vec![vec![vec![]; num_operands + 1]; num_operands];
        for begin in 0..num_operands {
            dp[begin][begin + 1].push(operands[begin]);
        }
        for len in 2..=num_operands {
            for begin in 0..=(num_operands - len) {
                let mut results = vec![];
                for split_len in 1..len {
                    let lhs = &dp[begin][begin + split_len];
                    let rhs = &dp[begin + split_len][begin + len];
                    let op  = ops[begin + split_len - 1];
                    for &lhs in lhs {
                        for &rhs in rhs {
                            results.push(match op {
                                b'+' => lhs + rhs,
                                b'-' => lhs - rhs,
                                b'*' => lhs * rhs,
                                _ => panic!("unreachable"),
                            });
                        }
                    }
                }
                dp[begin][begin + len] = results;
            }
        }
        dp[0][num_operands].clone()
    }

    fn parse_expr(expr: &str) -> (Vec<i32>, Vec<u8>) {
        let mut operands = vec![0i32; 0];
        let mut ops = vec![0u8; 0];
        let mut new_operand = true;
        for c in expr.bytes() {
            match c {
                b'0'..=b'9' =>
                    if new_operand {
                        operands.push((c - b'0') as _);
                        new_operand = false;
                    } else {
                        let num = operands.last_mut().unwrap();
                        *num *= 10;
                        *num += (c - b'0') as i32;
                    },
                b'+'|b'-'|b'*' => {
                    ops.push(c);
                    new_operand = true;
                },
                _ => panic!("invalid input"),
            }
        }
        (operands, ops)
    }
}

fn main() {
    use leetcode::cmp;
    assert!(cmp::eq_any_order(solution::main("2".into()), [2]));
    assert!(cmp::eq_any_order(solution::main("2-1".into()), [1]));
    assert!(cmp::eq_any_order(solution::main("2-1-1".into()), [0, 2]));
    assert!(cmp::eq_any_order(solution::main("2*3-4*5".into()), [-34, -14, -10, -10, 10]));
}
