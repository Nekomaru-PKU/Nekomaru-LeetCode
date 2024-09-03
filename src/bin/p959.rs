mod solution {
    use super::graph;

    pub fn main(grid: Vec<String>) -> i32 {
        let mut graph = graph::GraphBuilder::with_capacity(
            grid.len() *
            grid[0].len() * 8);
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
                        graph.insert_edge_undirected(vn, vw);
                        graph.insert_edge_undirected(vw, vs);
                        graph.insert_edge_undirected(vs, ve);
                        graph.insert_edge_undirected(ve, vn);
                    },
                    '/'  => {
                        graph.insert_edge_undirected(vn, vw);
                        graph.insert_edge_undirected(vs, ve);
                    },
                    '\\' => {
                        graph.insert_edge_undirected(vn, ve);
                        graph.insert_edge_undirected(vs, vw);
                    },
                    _ => panic!("invalid input"),
                }
            }
        }

        graph::algorithm::num_connected_components(&graph.build()) as _
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
    #![expect(dead_code)]

    use std::{
        collections::HashMap,
        hash::Hash,
        ops::Range,
    };

    pub struct Graph<T> {
        edges: Vec<(T, T)>,
        edge_ranges: HashMap<T, Range<usize>>,
    }

    pub struct GraphBuilder<T> {
        edges: Vec<(T, T)>,
    }

    impl<T> GraphBuilder<T> {
        pub const fn new() -> Self {
            Self { edges: Vec::new() }
        }

        pub fn with_capacity(capacity: usize) -> Self {
            Self { edges: Vec::with_capacity(capacity) }
        }
    }

    impl<T> GraphBuilder<T> {
        pub fn insert_edge(&mut self, from: T, to: T) {
            self.edges.push((from, to));
        }
    }

    impl<T: Clone> GraphBuilder<T> {
        pub fn insert_edge_undirected(&mut self, from: T, to: T) {
            self.edges.push((from.clone(), to.clone()));
            self.edges.push((to, from));
        }
    }

    impl<T: Clone + Eq + Ord + Hash> GraphBuilder<T> {
        pub fn build(self) -> Graph<T> {
            let mut edges = self.edges;
            edges.sort_unstable_by_key(|&(ref from, _)| from.clone());

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

            Graph { edges, edge_ranges }
        }
    }

    pub mod algorithm {
        use super::Graph;
        use std::{
            collections::HashSet,
            hash::Hash,
        };

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
                    for &(_, ref to) in &graph.edges[graph.edge_ranges[&node].clone()] {
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
