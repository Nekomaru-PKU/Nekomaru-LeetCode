mod solution {
    pub fn main(mut people: Vec<i32>, limit: i32) -> i32 {
        people.sort_unstable();
        let mut begin = 0;
        let mut end = people.len() - 1; // note that `end` is inclusive!!
        let mut boats = 0;
        while begin < end {
            if people[begin] + people[end] <= limit {
                begin += 1;
            }
            end -= 1;
            boats += 1;
        }
        if begin == end { // we have one person left.
            boats += 1;
        }
        boats
    }
}

use leetcode::print_time;

fn main() {
    assert_eq!(solution::main(vec![1, 2], 3), 1);
    assert_eq!(solution::main(vec![3, 2, 2, 1], 3), 3);
    assert_eq!(solution::main(vec![3, 5, 3, 4], 5), 4);

    assert_eq!(solution::main(vec![5, 1, 7, 4, 2, 4], 7), 4);

    assert_eq!(solution::main(vec![5, 5, 5, 5, 5], 5), 5);
    assert_eq!(solution::main(vec![1, 2, 3, 4, 5], 6), 3);
    assert_eq!(solution::main(vec![1, 2, 3, 4], 5), 2);

    let people = (1..=50000).map(|_| 50000).collect::<Vec<_>>();
    let result = print_time("perf", || solution::main(people, 50000));
    // in this case everyone is on its own boat.
    assert_eq!(result, 50000);

    let people = (1..50000).collect::<Vec<_>>();
    let result = print_time("perf", || solution::main(people, 50000));
    // in this case we pair `people[i]` with `people[50000 - i]` for each `i`
    // from 0 to 24999 and we have `people[25000]` left, so we need 25000 boats.
    assert_eq!(result, 25000);

    let people = (1..=50000).collect::<Vec<_>>();
    let result = print_time("perf", || solution::main(people, 50001));
    // in this case we pair `people[i]` with `people[50001 - i]` for each `i`
    // from 0 to 25000, so we need 25000 boats.
    assert_eq!(result, 25000);
}
