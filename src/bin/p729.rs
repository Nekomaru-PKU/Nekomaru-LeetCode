use core::ops::Range;

struct MyCalendar(Vec<Range<i32>>);

impl MyCalendar {
    fn new() -> Self {
        Self(Vec::with_capacity(1024))
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        let arr = &mut self.0;
        let Err(idx) = arr.binary_search_by_key(&start, |range| range.start) else {
            return false;
        };
        let prev_end   = if idx > 0         {arr[idx - 1].end} else {0};
        let next_start = if idx < arr.len() {arr[idx].start}   else {i32::MAX};
        if !(start >= prev_end && end <= next_start) {
            return false;
        }
        arr.insert(idx, start..end);
        true
    }
}

fn main() {
    let mut calendar = MyCalendar::new();
    assert!(calendar.book(10, 20));
    assert!(!calendar.book(15, 25));
    assert!(calendar.book(20, 30));
}
