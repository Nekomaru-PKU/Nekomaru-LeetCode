mod graph {
    #![allow(unused_imports)]

    pub use core::{
        Graph,
        GraphBuilder,
    };

    mod core {
        #![allow(dead_code)]

        pub struct Graph<E> {
            verts: Vec<usize>,
            edges: Vec<(usize, usize, E)>,
        }

        impl<E> Graph<E> {
            pub fn num_verts(&self) -> usize {
                self.verts.len()
            }

            pub fn edges_from(&self, vert: usize)
             -> impl Iterator<Item = &(usize, usize, E)> {
                let begin = self.verts[vert];
                let end = self.verts
                    .get(vert + 1)
                    .cloned()
                    .unwrap_or(self.edges.len());
                self.edges[begin..end].iter()
            }
        }

        pub struct GraphBuilder<E> {
            num_vert: usize,
            edges: Vec<(usize, usize, E)>,
        }

        impl<E> GraphBuilder<E> {
            pub fn new(num_vert: usize) -> Self {
                Self {
                    num_vert,
                    edges: Vec::new(),
                }
            }

            pub fn with_capacity(num_vert: usize, capacity: usize) -> Self {
                Self {
                    num_vert,
                    edges: Vec::with_capacity(capacity),
                }
            }
        }

        impl<E> GraphBuilder<E> {
            pub fn insert_edge(&mut self, from: usize, to: usize, weight: E) {
                debug_assert!(from < self.num_vert);
                debug_assert!(to   < self.num_vert);
                self.edges.push((from, to, weight));
            }
        }

        impl<E: Clone> GraphBuilder<E> {
            pub fn insert_edge_undirected(&mut self, from: usize, to: usize, weight: E) {
                debug_assert!(from < self.num_vert);
                debug_assert!(to   < self.num_vert);
                self.edges.push((from, to, weight.clone()));
                self.edges.push((to, from, weight));
            }
        }

        impl<E> GraphBuilder<E> {
            pub fn build(self) -> Graph<E> {
                let Self { num_vert, mut edges } = self;
                edges.sort_unstable_by_key(|&(from, _, _)| from);
                let mut verts = vec![0; num_vert + 1];
                for &(from, _, _) in &edges {
                    verts[from + 1] += 1;
                }
                for i in 0..num_vert {
                    verts[i + 1] += verts[i];
                }
                verts.pop();
                Graph { edges, verts }
            }
        }
    }
}

fn solution(
    n: i32,
    mut edges: Vec<Vec<i32>>,
    src: i32,
    dst: i32,
    target: i32) -> Vec<Vec<i32>> {
    use std::{
        collections::BinaryHeap,
        cmp::Ordering,
    };

    let graph = {
        let mut graph = graph::GraphBuilder::new(n as _);
        for edge in &edges {
            graph.insert_edge_undirected(
                edge[0] as _,
                edge[1] as _,
                edge[2]);
        }
        graph.build()
    };

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    struct DistToVert {
        vert: usize,
        dist: i32,
        unknowns: Vec<(usize, usize)>,
    }

    impl PartialOrd for DistToVert {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for DistToVert {
        fn cmp(&self, other: &Self) -> Ordering {
            self.dist
                .cmp(&other.dist)
                .reverse()
                .then(self.vert.cmp(&other.vert))
                .then(self.unknowns.cmp(&other.unknowns))
        }
    }

    let mut num_unknowns  = 0;
    let mut min_dist      = vec![(i32::MAX, vec![]); n as _];
    let mut min_dist_prev = vec![(i32::MAX, vec![]); n as _];
    let mut queue         = BinaryHeap::new();
    min_dist_prev[src as usize] = (0, vec![]);
    loop {
        if num_unknowns == 0 {
            min_dist[src as usize] = (0, vec![]);
            queue.push(DistToVert {
                vert: src as _,
                dist: 0,
                unknowns: vec![],
            })
        } else {
            for (vert, &(dist, ref unknowns)) in min_dist_prev.iter().enumerate() {
                if dist < i32::MAX {
                    for &(_, to, weight) in graph.edges_from(vert) {
                        if  weight < 0 &&
                            dist < min_dist[to].0 &&
                            !unknowns.contains(&(vert, to)) &&
                            !unknowns.contains(&(to, vert)) {
                            let mut unknowns = unknowns.clone();
                            unknowns.push((vert, to));
                            min_dist[to] = (dist, unknowns.clone());
                            queue.push(DistToVert {
                                vert: to,
                                dist,
                                unknowns,
                            });
                        }
                    }
                }
            }
        }

        while let Some(DistToVert { vert, dist, unknowns }) = queue.pop() {
            debug_assert_eq!(unknowns.len(), num_unknowns);
            for &(_, to, weight) in graph.edges_from(vert) {
                if weight >= 0 {
                    let new_dist = dist + weight;
                    if new_dist < min_dist[to].0 {
                        min_dist[to] = (new_dist, unknowns.clone());
                        queue.push(DistToVert {
                            vert: to,
                            dist: new_dist,
                            unknowns: unknowns.clone(),
                        });
                    }
                }
            }
        }

        dbg!(num_unknowns);
        dbg!(&min_dist);

        let (min_dist_to_dest, mut unknowns_to_dest) =
            min_dist[dst as usize].clone();
        if min_dist_to_dest != i32::MAX {
            debug_assert_eq!(unknowns_to_dest.len(), num_unknowns);
        }
        if num_unknowns == 0 {
            if min_dist_to_dest < target {
                return vec![];
            }
            if min_dist_to_dest == target {
                for edge in &mut edges {
                    if edge[2] == -1 {
                        edge[2] = 2_000_000_000;
                    }
                }
                return edges;
            }
        } else if min_dist_to_dest <= target - num_unknowns as i32 {
            unknowns_to_dest.sort_unstable();
            let extra_dist = target - min_dist_to_dest; // >= num_unknwon
            let weight_for_first_edge = extra_dist - (num_unknowns as i32 - 1);
            let weight_for_other_edges = 1;
            for edge in &mut edges {
                if  unknowns_to_dest.contains(&(edge[0] as _, edge[1] as _)) ||
                    unknowns_to_dest.contains(&(edge[1] as _, edge[0] as _)) {
                    if  unknowns_to_dest[0] == (edge[0] as _, edge[1] as _) ||
                        unknowns_to_dest[0] == (edge[1] as _, edge[0] as _) {
                        edge[2] = weight_for_first_edge;
                    } else {
                        edge[2] = weight_for_other_edges;
                    }
                } else if edge[2] == -1 {
                    edge[2] = 2_000_000_000;
                }
            }
            return edges;
        }

        if min_dist.iter().all(|&(min_dist, _)| min_dist == i32::MAX) {
            return vec![];
        }

        (min_dist, min_dist_prev) = (min_dist_prev, min_dist);
        min_dist.fill((i32::MAX, vec![]));
        num_unknowns += 1;
    }
}

fn main() {
    use leetcode::input::Input;
    dbg!((solution(
        5,
        [[4, 1, -1], [2, 0, -1], [0, 3, -1] ,[4, 3, -1]].input(),
        0,
        1,
        5),
        [[4, 1, 1], [2, 0, 1], [0, 3, 3] ,[4, 3, 1]]));
    dbg!((solution(
        3,
        [[0,1,-1],[0,2,5]].input(),
        0,
        2,
        6),
        [[0; 0]; 0]));
    dbg!((solution(
        4,
        [[1, 0, 4], [1, 2, 3], [2, 3, 5] ,[0, 3, -1]].input(),
        0,
        2,
        6),
        [[1, 0, 4], [1, 2, 3], [2, 3, 5] ,[0, 3, 1]]));
}
