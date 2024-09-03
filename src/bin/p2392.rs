mod graph {
    pub fn topology_sort<E>(n: usize, edges: E) -> Option<Vec<usize>>
    where
        E: Iterator<Item = (usize, usize)> {
        use std::collections::HashSet;

        let mut edges = edges.collect::<Vec<_>>();
        edges.sort_unstable_by_key(|&(from, _)| from);

        let mut num_out_edges = vec![0; n];
        for &(from, _) in &edges {
            num_out_edges[from] += 1;
        }

        let mut num_out_edges_prefix_sum = vec![0; n + 1];
        for i in 1..=n {
            num_out_edges_prefix_sum[i] =
                num_out_edges_prefix_sum[i - 1] +
                num_out_edges[i - 1];
        }

        let mut edge_ranges = vec![(0, 0); n];
        for i in 0..n {
            edge_ranges[i] = (
                num_out_edges_prefix_sum[i],
                num_out_edges_prefix_sum[i + 1],
            );
        }

        let mut vertices = (0..n)
            .collect::<HashSet<_>>();
        let mut vertices_no_out_edge = (0..n)
            .filter(|&i| edge_ranges[i].0 == edge_ranges[i].1)
            .collect::<Vec<_>>();

        let mut result = Vec::new();
        while let Some(removing_vertex) = vertices_no_out_edge.pop() {
            // remove all edges to `removing_vertex`:
            #[expect(clippy::iter_over_hash_type)]
            for &i in &vertices {
                let (ref edge_begin, ref mut edge_end) = edge_ranges[i];
                if edge_begin < edge_end {
                    let mut next = Some(0);
                    while let Some(next) = {
                        next = edges[*edge_begin..*edge_end]
                            .iter()
                            .enumerate()
                            .skip(next.unwrap())
                            .find(|&(_, &(_, to))| to == removing_vertex)
                            .map(|(i, _)| i);
                        next
                    } {
                        *edge_end -= 1;
                        edges.swap(edge_begin + next, *edge_end);
                    }

                    if edge_begin == edge_end {
                        vertices_no_out_edge.push(i);
                    }
                }
            }
            edge_ranges[removing_vertex] = (0, 0);
            vertices.remove(&removing_vertex);
            result.push(removing_vertex);
        }

        vertices.is_empty().then(|| {
            result.reverse();
            result
        })
    }
}

fn solution(
    k: i32,
    row_conditions: Vec<Vec<i32>>,
    column_conditions: Vec<Vec<i32>>,
) -> Vec<Vec<i32>> {
    let k = k as usize;

    let Some(nums_by_row) =
        graph::topology_sort(
            k,
            row_conditions.into_iter().map(|vec| (
                vec[0] as usize - 1,
                vec[1] as usize - 1)))
    else { return Vec::new() };
    let Some(nums_by_column) =
        graph::topology_sort(
            k,
            column_conditions.into_iter().map(|vec| (
                vec[0] as usize - 1,
                vec[1] as usize - 1)))
    else { return Vec::new() };

    let mut column_of = vec![0; k + 1];
    for (column, &num) in nums_by_column.iter().enumerate() {
        column_of[num] = column;
    }

    let mut mat = vec![vec![0; k]; k];
    for (i, num) in nums_by_row.iter().copied().enumerate() {
        mat[i][column_of[num]] = num as i32 + 1;
    }

    mat
}

fn main() {
    assert_eq!(solution(
        3,
        vec![vec![1, 2], vec![3, 2]],
        vec![vec![2, 1], vec![3, 2]]),
        vec![
            vec![0, 0, 1],
            vec![3, 0, 0],
            vec![0, 2, 0],
        ]);
}
