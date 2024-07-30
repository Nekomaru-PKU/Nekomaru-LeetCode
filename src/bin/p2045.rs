fn solution(
    n: i32,
    edges: Vec<Vec<i32>>,
    edge_time: i32,
    signal_change_time: i32)
 -> i32 {
    let n = n as usize;
    let twice_signal_change_time = signal_change_time * 2;

    let (out_buf, out_len) = {
        let mut edges = edges.iter()
            .flat_map(|edge| [
                (edge[0], edge[1]),
                (edge[1], edge[0])
            ])
            .map(|(from, to)| (
                from as usize - 1,
                to   as usize - 1))
            .collect::<Vec<_>>();
        edges.sort_unstable();

        let mut out_count = vec![0; n];
        for &(from, _) in &edges {
            out_count[from] += 1;
        }

        let out_buf = edges.iter()
            .map(|&(_, to)| to)
            .collect::<Vec<_>>();
        (out_buf, out_count)
    };

    let out = {
        let mut acc = 0;
        let mut out = Vec::with_capacity(n);
        for &len in &out_len {
            out.push(&out_buf[acc..(acc + len)]);
            acc += len;
        }
        out
    };

    fn update_min_and_second_min<T: Default + Ord>(
        min: &mut T,
        second_min: &mut T,
        value: T) {
        use std::mem;
        if &value < min {
            (*min, *second_min) = (value, mem::take(min));
        } else if &value > min && &value < second_min {
            *second_min = value;
        }
    }

    let mut min_time = vec![(i32::MAX, i32::MAX); n];

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct Arrival {
        vert: usize,
        time: i32,
    }

    use std::cmp::Ordering;

    impl PartialOrd for Arrival {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for Arrival {
        fn cmp(&self, other: &Self) -> Ordering {
            self.time
                .cmp(&other.time)
                .reverse()
                .then(self.vert.cmp(&other.vert))
        }
    }

    use std::collections::BinaryHeap;

    let mut queue = BinaryHeap::new();
    queue.push(Arrival { vert: 0, time: 0 });

    while let Some(Arrival { vert, time: arrival_time }) = queue.pop() {
        #[cfg(debug_assertions)]
        println!("arrive at vert {vert} at time {arrival_time}");

        let time_since_last_green = arrival_time % twice_signal_change_time;
        let departure_time = if time_since_last_green >= signal_change_time {
            arrival_time + twice_signal_change_time - time_since_last_green
        } else {
            arrival_time
        };
        let next_arrival_time = departure_time + edge_time;
        if next_arrival_time < min_time[n - 1].1 {
            for &next in out[vert] {
                if next_arrival_time < min_time[next].1 {
                    let &mut (ref mut min, ref mut second_min) = &mut min_time[next];
                    update_min_and_second_min(min, second_min, next_arrival_time);
                    queue.push(Arrival { vert: next, time: next_arrival_time });
                }
            }
        }
    }

    min_time[n - 1].1
}

fn main() {
    assert_eq!(solution(
        5,
        [[1, 2], [1, 3], [1, 4], [3, 4], [4, 5]].iter()
            .map(|vec| vec.to_vec())
            .collect(),
        3,
        5),
        13);
    assert_eq!(solution(
        2,
        [[1, 2]].iter()
            .map(|vec| vec.to_vec())
            .collect(),
        3,
        2),
        11);
    assert_eq!(solution(
        6,
        [[1, 2], [1, 3], [2, 4], [3, 5], [5, 4], [4, 6]].iter()
            .map(|vec| vec.to_vec())
            .collect(),
        3,
        100),
        12);
}
