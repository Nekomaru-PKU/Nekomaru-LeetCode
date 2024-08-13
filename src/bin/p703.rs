use std::collections::BinaryHeap;
use std::cmp::Reverse;

struct KthLargest{
    heap: BinaryHeap<Reverse<i32>>,
}

impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut heap = BinaryHeap::from(
            vec![Reverse(i32::MIN); k as usize]);
        for num in nums {
            if num > heap.peek().unwrap().0 {
                // here actually re-heapify twice is not
                // necessary, but the standard library
                // does not provide a method to replace
                // top element of the heap and re-heapify.
                heap.pop();
                heap.push(Reverse(num));
            }
        }
        Self { heap }
    }

    fn add(&mut self, num: i32) -> i32 {
        if let Some(&Reverse(min)) = self.heap.peek() {
            if num > min {
                self.heap.pop();
                self.heap.push(Reverse(num));
                self.heap.peek().unwrap().0
            } else {
                min
            }
        } else {
            self.heap.push(Reverse(num));
            num
        }
    }
}

fn main() {
    let mut kth_largest = KthLargest::new(3, vec![4,5,8,2]);
    assert_eq!(kth_largest.add(3), 4);
    assert_eq!(kth_largest.add(5), 5);
    assert_eq!(kth_largest.add(10), 5);
    assert_eq!(kth_largest.add(9), 8);
    assert_eq!(kth_largest.add(4), 8);

    let mut kth_largest = KthLargest::new(1, vec![]);
    assert_eq!(kth_largest.add(-3), -3);
    assert_eq!(kth_largest.add(-2), -2);
    assert_eq!(kth_largest.add(-4), -2);
    assert_eq!(kth_largest.add(0), 0);
    assert_eq!(kth_largest.add(4), 4);

    let mut kth_largest = KthLargest::new(2, vec![0]);
    assert_eq!(kth_largest.add(-1), -1);
    assert_eq!(kth_largest.add(1), 0);
    assert_eq!(kth_largest.add(-2), 0);
    assert_eq!(kth_largest.add(-4), 0);
    assert_eq!(kth_largest.add(3), 1);
}
