mod solution {
    pub fn main(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let mut moves = 0;
        let mut next = nums[0];
        for &num in &nums {
            if num < next {
                moves += next - num;
            } else {
                next = num;
            }
            next += 1;
        }
        moves
    }
}

fn main() {
    assert_eq!(solution::main(vec![1, 2, 2]), 1);
    assert_eq!(solution::main(vec![3, 2, 1, 2, 1, 7]), 6);
}
