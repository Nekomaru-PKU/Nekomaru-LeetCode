mod solution {
    pub fn main(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;

        let num_even_before = {
            let mut vec = Vec::with_capacity(nums.len());
            let mut num = 0;
            for i in nums {
                if i & 1 == 1 {
                    vec.push(num);
                    num = 0;
                } else {
                    num += 1;
                }
            }
            vec.push(num);
            vec.shrink_to_fit();
            vec
        };
        let num_odd = num_even_before.len() - 1;
        if num_odd < k {
            return 0;
        }

        let mut count = 0;
        for begin in 0..=(num_odd - k) {
            let end = begin + k;
            count += (num_even_before[begin] + 1) * (num_even_before[end] + 1);
        }
        count
    }
}

fn main() {
    assert_eq!(solution::main(vec![1, 1, 2, 1, 1], 3), 2);
    assert_eq!(solution::main(vec![2, 4, 6], 1), 0);
    assert_eq!(solution::main(vec![2, 2, 2, 1, 2, 2, 1, 2, 2, 2], 2), 16);
}
