mod graph {
    pub use core::{
        Graph,
        GraphBuilder,
    };

    mod core {
        #![expect(dead_code)]

        use std::{
            collections::HashMap,
            hash::Hash,
            ops::Range,
        };

        pub struct Graph<T> {
            verts: HashMap<T, Range<usize>>,
            edges: Vec<(T, T)>,
        }

        impl<T> Graph<T> {
            pub fn verts(&self)
            -> impl Iterator<Item = &T> + '_ {
                self.verts.keys()
            }
        }

        impl<T: Eq + Hash> Graph<T> {
            pub fn edges_from(&self, vert: &T)
            -> impl Iterator<Item = &(T, T)> {
                self.edges[self.verts[vert].clone()].iter()
            }
        }

        pub struct GraphBuilder<T> {
            verts: Vec<T>,
            edges: Vec<(T, T)>,
        }

        impl<T> GraphBuilder<T> {
            pub const fn new() -> Self {
                Self {
                    verts: Vec::new(),
                    edges: Vec::new(),
                }
            }
        }

        impl<T> GraphBuilder<T> {
            pub fn insert_vert(&mut self, vert: T) {
                self.verts.push(vert);
            }

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

                let mut verts = HashMap::new();
                for i in 1..edges.len() {
                    if edges[i].0 != edges[i - 1].0 {
                        verts
                            .entry(edges[i - 1].0.clone())
                            .or_insert(0..edges.len())
                            .end = i;
                        verts
                            .entry(edges[i].0.clone())
                            .or_insert(0..edges.len())
                            .start = i;
                    }
                }

                for vert in self.verts {
                    verts.entry(vert).or_insert(0..0);
                }

                Graph { verts, edges }
            }
        }
    }

    pub mod algorithm {
        use std::{
            collections::HashSet,
            hash::Hash,
        };

        use super::Graph;

        pub fn num_connected_components<T: Eq + Hash>(graph: &Graph<T>) -> usize {
            let mut unvisited = graph.verts().collect::<HashSet<_>>();
            let mut stack = Vec::new();
            let mut num = 0;
            while let Some(&start) = unvisited.iter().next() {
                num += 1;
                stack.push(start);
                while let Some(vert) = stack.pop() {
                    unvisited.remove(&vert);
                    for &(_, ref to) in graph.edges_from(vert) {
                        if unvisited.contains(to) {
                            stack.push(to);
                        }
                    }
                }
            }
            num
        }
    }
}

fn solution(stones: Vec<Vec<i32>>) -> i32 {
    let mut graph = graph::GraphBuilder::new();

    use std::collections::HashMap;
    let mut rows = HashMap::<_, Vec<_>>::new();
    let mut cols = HashMap::<_, Vec<_>>::new();

    for stone in &stones {
        let (i, j) = (stone[0], stone[1]);
        graph.insert_vert((i, j));
        rows.entry(i).or_default().push((i, j));
        cols.entry(j).or_default().push((i, j));
    }

    for (_, group) in Iterator::chain(rows.iter(), cols.iter()) {
        for i in 0..(group.len() - 1) {
            graph.insert_edge_undirected(
                group[i],
                group[i + 1]);
        }
        graph.insert_edge_undirected(
            group[0],
            group[group.len() - 1]);
    }

    let graph = graph.build();
    let num = graph::algorithm::num_connected_components(&graph);
    (stones.len() - num) as _
}

fn main() {
    use leetcode::input::Input;
    assert_eq!(solution([[0, 0], [0, 1], [1, 0], [1, 2], [2, 1], [2, 2]].input()), 5);
    assert_eq!(solution([[0, 0], [0, 2], [1, 1], [2, 0], [2, 2]].input()), 3);
    assert_eq!(solution([[0, 0]].input()), 0);
}
