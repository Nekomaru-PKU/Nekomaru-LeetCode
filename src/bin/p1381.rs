struct CustomStack {
    vec: Vec<i32>,
    max_size: usize,
}

impl CustomStack {
    fn new(max_size: i32) -> Self {
        Self {
            vec: Vec::with_capacity(1024),
            max_size: max_size as _,
        }
    }

    fn push(&mut self, x: i32) {
        if self.vec.len() < self.max_size {
            self.vec.push(x);
        }
    }

    fn pop(&mut self) -> i32 {
        self.vec.pop().unwrap_or(-1)
    }

    fn increment(&mut self, k: i32, val: i32) {
        let k = (k as usize).min(self.vec.len());
        for elem in &mut self.vec[..k] {
            *elem += val;
        }
    }
}

fn main() {
    let mut stack = CustomStack::new(3);
    stack.push(1);
    stack.push(2);
    assert_eq!(stack.pop(), 2);
    stack.push(2);
    stack.push(3);
    stack.push(4);
    stack.increment(5, 100);
    stack.increment(2, 100);
    assert_eq!(stack.pop(), 103);
    assert_eq!(stack.pop(), 202);
    assert_eq!(stack.pop(), 201);
    assert_eq!(stack.pop(), -1);
}
