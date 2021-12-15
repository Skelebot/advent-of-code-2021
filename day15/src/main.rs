use std::{collections::BinaryHeap, io::BufRead};

fn main() {
    let graph: Vec<Vec<u8>> = input::read_file("puzzles/day15.txt")
        .lines()
        .map(|l| l.unwrap().chars().map(|c| c as u8 - b'0').collect())
        .collect();

    println!("solution 1: {}", dijkstra(&graph));

    let graph = extend(&graph);
    println!("solution 2: {}", dijkstra(&graph));
}

#[derive(Eq, PartialEq, Debug)]
struct Vertex {
    location: (usize, usize),
    dist: usize,
}

impl PartialOrd for Vertex {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// Reverse Ord to create min binary heap
impl Ord for Vertex {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.dist.cmp(&self.dist)
    }
}

pub fn dijkstra(graph: &[Vec<u8>]) -> usize {
    let max = graph.len();
    let mut dist = vec![vec![usize::MAX; max]; max];
    let mut prev = vec![vec![None; max]; max];

    let mut heap = BinaryHeap::new();
    heap.push(Vertex {
        location: (0, 0),
        dist: 0,
    });
    dist[0][0] = 0;

    while let Some(u) = heap.pop() {
        if u.location == (max - 1, max - 1) {
            // Reached end
            let mut path = Vec::new();
            let mut current = prev[max - 1][max - 1];
            path.push((max - 1, max - 1));
            while let Some(pv) = current {
                path.push(pv);
                current = prev[pv.1][pv.0];
            }
            path.pop();
            path.reverse();
            return path.iter().map(|(x, y)| graph[*y][*x] as usize).sum();
        }

        if u.dist > dist[u.location.1][u.location.0] {
            continue;
        }

        for v in neighbors(u.location, max) {
            let new = Vertex {
                location: v,
                dist: u.dist + graph[v.1][v.0] as usize,
            };
            if new.dist < dist[new.location.1][new.location.0] {
                dist[new.location.1][new.location.0] = new.dist;
                prev[new.location.1][new.location.0] = Some(u.location);
                heap.push(new);
            }
        }
    }

    0
}

fn neighbors(loc: (usize, usize), max: usize) -> impl Iterator<Item = (usize, usize)> {
    [
        (loc.0.checked_sub(1), Some(loc.1)),
        (
            if loc.0 + 1 == max {
                None
            } else {
                Some(loc.0 + 1)
            },
            Some(loc.1),
        ),
        (Some(loc.0), loc.1.checked_sub(1)),
        (
            Some(loc.0),
            if loc.1 + 1 == max {
                None
            } else {
                Some(loc.1 + 1)
            },
        ),
    ]
    .into_iter()
    .filter(|(a, b)| a.is_some() && b.is_some())
    .map(|(a, b)| (a.unwrap(), b.unwrap()))
}

fn extend(map: &[Vec<u8>]) -> Vec<Vec<u8>> {
    let mut extended = Vec::with_capacity(map.len() * 5);

    for y in 0..map.len() * 5 {
        let map_row = &map[y % map.len()];
        let vert_chunk = y / map.len();

        let mut new_row = Vec::with_capacity(map.len() * 5);
        for x in 0..5 {
            new_row.extend(map_row.iter().map(|c| digit_add(c, vert_chunk as u8 + x)))
        }
        extended.push(new_row);
    }

    extended
}

fn digit_add(a: &u8, b: u8) -> u8 {
    if a + b > 9 {
        (a + b) - 9
    } else {
        a + b
    }
}

#[test]
fn test() {
    let graph_str = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";
    let graph: Vec<Vec<u8>> = graph_str
        .lines()
        .map(|l| l.chars().map(|c| c as u8 - b'0').collect())
        .collect();

    assert_eq!(dijkstra(&graph), 40);

    let graph = extend(&graph);
    assert_eq!(dijkstra(&graph), 315);
}
