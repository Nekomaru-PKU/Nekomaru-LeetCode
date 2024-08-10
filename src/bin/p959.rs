mod solution {
    use super::graph;

    pub fn main(grid: Vec<String>) -> i32 {
        let mut edges = Vec::with_capacity(
            grid.len() *
            grid.first().unwrap().len() * 8);
        for (row, s) in grid.iter().enumerate() {
            for (col, c) in s.chars().enumerate() {
                let row = row as _;
                let col = col as _;
                let vn = vertex_id(row, col, Side::N);
                let vw = vertex_id(row, col, Side::W);
                let vs = vertex_id(row, col, Side::S);
                let ve = vertex_id(row, col, Side::E);
                match c {
                    ' ' => {
                        edges.push((vn, vw));
                        edges.push((vw, vs));
                        edges.push((vs, ve));
                        edges.push((ve, vn));
                    },
                    '/'  => {
                        edges.push((vn, vw));
                        edges.push((vs, ve));
                    },
                    '\\' => {
                        edges.push((vn, ve));
                        edges.push((vs, vw));
                    },
                    _ => unreachable!("invalid input"),
                }
            }
        }
        graph::algorithm::num_connected_components(
            &graph::Graph::from_edges(
                edges.iter().cloned().chain(
                    edges.iter().map(|&(from, to)| (to, from))))
        ) as _
    }

    enum Side { N, W, S, E }

    fn vertex_id(row: u8, col: u8, side: Side) -> u16 {
        match side {
            Side::N => ((row as u16) << 8) + ((col as u16) << 1),
            Side::W => ((row as u16) << 8) + ((col as u16) << 1) + 1,
            Side::S => vertex_id(row + 1, col, Side::N),
            Side::E => vertex_id(row, col + 1, Side::W),
        }
    }
}

mod graph {
    use std::{
        collections::{
            HashMap,
            HashSet,
        },
        hash::Hash,
        ops::*
    };

    pub struct Graph<T> {
        edges: Vec<(T, T)>,
        edge_ranges: HashMap<T, Range<usize>>,
    }

    impl<T: Clone + Eq + Ord + Hash> Graph<T> {
        pub fn from_edges(edges: impl Iterator<Item = (T, T)>) -> Self {
            let mut edges = edges.collect::<Vec<_>>();
            edges.sort_unstable_by_key(|(from, _)| from.clone());

            let mut edge_ranges = HashMap::new();
            for i in 1..edges.len() {
                if edges[i].0 != edges[i - 1].0 {
                    edge_ranges
                        .entry(edges[i - 1].0.clone())
                        .or_insert(0..edges.len())
                        .end = i;
                    edge_ranges
                        .entry(edges[i].0.clone())
                        .or_insert(0..edges.len())
                        .start = i;
                }
            }

            edges.shrink_to_fit();
            edge_ranges.shrink_to_fit();

            Self { edges, edge_ranges }
        }
    }

    pub mod algorithm {
        use super::*;

        pub fn num_connected_components<T: Clone + Eq + Hash>(graph: &Graph<T>) -> usize {
            let mut unvisited = graph
                .edge_ranges
                .keys()
                .cloned()
                .collect::<HashSet<_>>();

            let mut stack = Vec::new();
            let mut count = 0;
            while let Some(start_node) = unvisited.iter().next().cloned() {
                count += 1;
                stack.push(start_node);
                while let Some(node) = stack.pop() {
                    unvisited.remove(&node);
                    for (_, to) in &graph.edges[graph.edge_ranges[&node].clone()] {
                        if unvisited.contains(to) {
                            stack.push(to.clone());
                        }
                    }
                }
            }
            count
        }
    }
}

fn main() {
    use leetcode::input::Input;
    assert_eq!(solution::main([" /", "/ "].input()), 2);
    assert_eq!(solution::main([" /", "  "].input()), 1);
    assert_eq!(solution::main(["/\\", "\\/"].input()), 5);
}
