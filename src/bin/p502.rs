fn solution(k: i32, w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
    use std::collections::BinaryHeap;

    // we gather and sort all projects by starting captical in ascending order
    let mut projects = capital.into_iter().zip(profits).collect::<Vec<_>>();
    projects.sort_unstable_by_key(|(requirement, _)| *requirement);

    let mut project_queue = BinaryHeap::<i32>::with_capacity(projects.len());
    let mut capital = w;
    let mut next_project = 0;
    for _ in 0..k {
        loop {
            if next_project >= projects.len() {
                break;
            }
            let &(requirement, profit) = &projects[next_project];
            if requirement > capital {
                break;
            }
            project_queue.push(profit);
            next_project += 1;
        }

        if let Some(profit) = project_queue.pop() {
            capital += profit;
        } else {
            // we're done. there's nothing we can do.
            break;
        }
    }

    capital
}

fn main() {
    // examples on leetcode
    assert_eq!(solution(2, 0, vec![1, 2, 3], vec![0, 1, 1]), 4);
    assert_eq!(solution(3, 0, vec![1, 2, 3], vec![0, 1, 2]), 6);

    // edge case: not enough capital for k projects
    assert_eq!(solution(2, 0, vec![1, 2, 3], vec![1, 1, 1]), 0);
}
