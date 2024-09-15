mod graph {
    #![expect(dead_code)]

    use std::ops::Range;

    pub struct Graph {
        num_vert: u32,
        edges: Vec<(u32, u32, f64)>,
        edge_ranges: Vec<Range<usize>>,
    }

    pub struct GraphBuilder {
        num_vert: u32,
        edges: Vec<(u32, u32, f64)>,
    }

    impl GraphBuilder {
        pub const fn new(num_vert: u32) -> Self {
            Self { num_vert, edges: Vec::new() }
        }

        pub fn with_capacity(num_vert: u32, capacity: usize) -> Self {
            Self { num_vert, edges: Vec::with_capacity(capacity) }
        }

        pub fn insert_edge(&mut self, from: u32, to: u32, weight: f64) {
            debug_assert!(from < self.num_vert);
            debug_assert!(to   < self.num_vert);
            self.edges.push((from, to, weight));
        }

        pub fn insert_edge_undirected(&mut self, from: u32, to: u32, weight: f64) {
            debug_assert!(from < self.num_vert);
            debug_assert!(to   < self.num_vert);
            self.edges.push((from, to, weight));
            self.edges.push((to, from, weight));
        }

        pub fn build(self) -> Graph {
            let Self { num_vert, mut edges } = self;
            edges.sort_unstable_by_key(|&(from, _, _)| from);

            let mut edge_ranges = vec![0..edges.len(); num_vert as _];
            for i in 1..edges.len() {
                if edges[i].0 != edges[i - 1].0 {
                    edge_ranges[edges[i - 1].0 as usize].end = i;
                    edge_ranges[edges[i].0     as usize].start = i;
                }
            }

            edges.shrink_to_fit();
            edge_ranges.shrink_to_fit();

            Graph { num_vert, edges, edge_ranges }
        }
    }

    pub mod algorithm {
        use std::{
            collections::BinaryHeap,
            cmp::Ordering,
        };

        use super::Graph;

        pub fn shortest_path(
            graph: &Graph,
            src: u32,
            dst: u32) -> f64 {
            #[derive(Debug, Clone, Copy, PartialEq)]
            struct DistToVert {
                vert: u32,
                dist: f64,
            }

            impl Eq  for DistToVert {}
            impl Ord for DistToVert {
                fn cmp(&self, other: &Self) -> Ordering {
                    self.dist.total_cmp(&other.dist)
                        .reverse()
                        .then(self.vert.cmp(&other.vert))
                }
            }

            impl PartialOrd for DistToVert {
                fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                    Some(self.cmp(other))
                }
            }

            let mut min_dist = vec![f64::INFINITY; graph.num_vert as _];
            min_dist[src as usize] = 0.0;

            let mut heap = BinaryHeap::new();
            heap.push(DistToVert { vert: src, dist: 0.0 });

            while let Some(DistToVert { dist, vert }) = heap.pop() {
                for &(_, next, weight) in
                    &graph.edges[graph.edge_ranges[vert as usize].clone()] {
                    let new_dist = dist + weight;
                    let min_dist = &mut min_dist[next as usize];
                    if new_dist < *min_dist {
                        *min_dist = new_dist;
                        heap.push(DistToVert {
                            vert: next,
                            dist: dist + weight,
                        });
                    }
                }
            }

            min_dist[dst as usize]
        }
    }
}

fn solution(
    n: i32,
    edges: Vec<Vec<i32>>,
    weights: Vec<f64>,
    src: i32,
    dst: i32) -> f64 {
    use crate::graph;
    let mut graph = graph::GraphBuilder::new(n as _);
    for ((from, to), weight) in
        Iterator::zip(
            edges
                .iter()
                .map(|vec| (vec[0] as _, vec[1] as _)),
            weights
                .iter()
                .map(|weight| -weight.ln())) {
        graph.insert_edge_undirected(from, to, weight);
    }

    let graph = graph.build();
    let min_dist = graph::algorithm::shortest_path(
        &graph,
        src as _,
        dst as _);
    if min_dist != f64::INFINITY {
        (-min_dist).exp()
    } else {
        0.0
    }
}

fn main() {
    #![expect(clippy::float_cmp)]
    assert_eq!(solution(
        3,
        vec![vec![0, 1], vec![1, 2], vec![0, 2]],
        vec![0.5,0.5,0.2],
        0,
        2), 0.25);
    assert_eq!(solution(
        3,
        vec![vec![0, 1], vec![1, 2], vec![0, 2]],
        vec![0.5,0.5,0.3],
        0,
        2), 0.3);
    assert_eq!(solution(
        3,
        vec![vec![0, 1]],
        vec![0.5],
        0,
        2), 0.0);
}
