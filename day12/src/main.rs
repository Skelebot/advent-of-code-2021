use std::{fmt::Debug, str::FromStr};

#[derive(Debug, PartialEq)]
struct Edge<T: Debug + PartialEq>(T, T);
impl FromStr for Edge<String> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hyphen = s.find('-').unwrap();
        let start = s[..hyphen].trim().to_string();
        let end = s[hyphen + 1..].trim().to_string();
        Ok(Edge(start, end))
    }
}

struct CaveGraph {
    start: usize,
    end: usize,
    lookup: Vec<String>,
    matrix: Vec<Vec<bool>>,
}

impl CaveGraph {
    pub fn new(edges: &[Edge<String>]) -> Self {
        let assigned = Self::assign_vertex_ids(edges);
        let start = assigned.iter().position(|c| *c == "start").unwrap();
        let end = assigned.iter().position(|c| *c == "end").unwrap();

        let edges = Self::optimize_edge_list(edges, &assigned);

        let mut matrix = vec![vec![false; assigned.len()]; assigned.len()];
        for edge in edges {
            matrix[edge.0][edge.1] = true;
            matrix[edge.1][edge.0] = true;
        }

        CaveGraph {
            start,
            end,
            lookup: assigned,
            matrix,
        }
    }

    fn assign_vertex_ids(edges: &[Edge<String>]) -> Vec<String> {
        let mut vertices = Vec::with_capacity(edges.len() / 2);
        for edge in edges {
            if !vertices.contains(&edge.0) {
                vertices.push(edge.0.clone());
            }
            if !vertices.contains(&edge.1) {
                vertices.push(edge.1.clone());
            }
        }
        vertices
    }

    fn optimize_edge_list(edges: &[Edge<String>], lookup: &[String]) -> Vec<Edge<usize>> {
        let mut new = Vec::with_capacity(edges.len());
        for edge in edges {
            let start = lookup.iter().position(|c| *c == edge.0).unwrap();
            let end = lookup.iter().position(|c| *c == edge.1).unwrap();
            let edge = Edge(start, end);
            if !new.contains(&edge) {
                new.push(Edge(start, end));
            }
        }

        new
    }

    fn is_small(&self, vid: usize) -> bool {
        self.lookup[vid]
            .chars()
            .next()
            .unwrap()
            .is_ascii_lowercase()
    }

    fn neighbors(&self, v: usize) -> impl Iterator<Item = usize> + '_ {
        self.matrix[v]
            .iter()
            .enumerate()
            .filter(|(_, b)| **b)
            .map(|(i, _)| i)
    }

    pub fn solve1(&self) -> usize {
        let seen = vec![false; self.lookup.len()];
        self.find_paths1(self.start, &seen)
    }

    pub fn solve2(&self) -> usize {
        let seen = vec![false; self.lookup.len()];
        self.find_paths2(self.start, &seen, true)
    }

    fn find_paths1(&self, v: usize, seen: &[bool]) -> usize {
        if v == self.end {
            return 1;
        }
        let mut paths = 0;
        let mut seen = seen.to_owned();
        if seen[v] {
            if v == self.start {
                return 0;
            }
            if self.is_small(v) {
                return 0;
            }
        }
        for n in self.neighbors(v) {
            seen[v] = true;
            paths += self.find_paths1(n, &seen);
        }
        paths
    }

    fn find_paths2(&self, v: usize, seen: &[bool], mut twice: bool) -> usize {
        if v == self.end {
            return 1;
        }
        let mut seen = seen.to_owned();
        let mut paths = 0;
        if seen[v] {
            if v == self.start {
                return 0;
            }
            if self.is_small(v) {
                if twice {
                    twice = false;
                } else {
                    return 0;
                }
            }
        }
        for n in self.neighbors(v) {
            seen[v] = true;
            paths += self.find_paths2(n, &seen, twice);
        }
        paths
    }
}

fn main() {
    let edges: Vec<Edge<String>> = input::read_lines("puzzles/day12.txt");
    let graph = CaveGraph::new(&edges);
    println!("solution 1: {}", graph.solve1());
    println!("solution 2: {}", graph.solve2());
}

#[test]
fn test_paths() {
    let edges: Vec<Edge<String>> = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc"
        .lines()
        .map(|l| l.parse().unwrap())
        .collect();
    let graph = CaveGraph::new(&edges);
    assert_eq!(graph.solve1(), 19);
    assert_eq!(graph.solve2(), 103);
}
