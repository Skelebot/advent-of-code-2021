use std::io::BufRead;

struct Map<const RS: usize> {
    pub points: Vec<[u8; RS]>,
}

impl<const RS: usize> Map<RS> {
    // [left, right, up, down]
    pub fn adjacent(&self, coords: (usize, usize)) -> [Option<u8>; 4] {
        let mut neighbors = [None; 4];
        if coords.0 != 0 {
            if let Some(n) = self.points[coords.1].get(coords.0 - 1) {
                neighbors[0] = Some(*n);
            }
        }
        if let Some(n) = self.points[coords.1].get(coords.0 + 1) {
            neighbors[1] = Some(*n);
        }
        if coords.1 != 0 {
            if let Some(row) = self.points.get(coords.1 - 1) {
                neighbors[2] = Some(row[coords.0]);
            }
        }
        if let Some(row) = self.points.get(coords.1 + 1) {
            neighbors[3] = Some(row[coords.0]);
        }
        neighbors
    }
    pub fn get_mut(&mut self, coords: (usize, usize)) -> Option<&mut u8> {
        if let Some(row) = self.points.get_mut(coords.1) {
            row.get_mut(coords.0)
        } else {
            None
        }
    }
}

fn main() {
    let input: Vec<[u8; 100]> = input::read_file("puzzles/day9.txt")
        .lines()
        .map(|l| {
            let mut row = [0; 100];
            l.unwrap()
                .chars()
                .map(|c| c as u8 - b'0')
                .enumerate()
                .for_each(|(idx, c)| row[idx] = c);
            row
        })
        .collect();
    let mut map = Map { points: input };
    println!("solution 1: {}", solve1(&map));
    println!("solution 2: {}", solve2(&mut map));
}

fn solve1<const RS: usize>(map: &Map<RS>) -> usize {
    let mut levels = 0;
    for y in 0..map.points.len() {
        for x in 0..RS {
            let num = map.points[y][x];
            if map.adjacent((x, y)).into_iter().flatten().all(|c| c > num) {
                levels += num as usize + 1;
            }
        }
    }
    levels
}
fn solve2<const RS: usize>(map: &mut Map<RS>) -> usize {
    let mut fill = 10;
    for y in 0..map.points.len() {
        for x in 0..RS {
            let val = map.points[y][x];
            if val < 9 {
                fill += 1;
                flood_fill((x, y), map, fill);
            }
        }
    }
    let mut basins: Vec<usize> = vec![0; fill as usize - 9];
    map.points.iter().flat_map(|a| a.iter()).for_each(|c| {
        if *c >= 10 {
            basins[*c as usize - 10] += 1
        }
    });
    basins.sort_unstable();
    basins.iter().rev().take(3).product()
}

fn flood_fill<const RS: usize>(coords: (usize, usize), map: &mut Map<RS>, num: u8) {
    std::io::Write::flush(&mut std::io::stdout().lock()).unwrap();
    if let Some(node) = map.get_mut(coords) {
        if *node == 9 || *node >= num {
            return;
        }
        *node = num;
        flood_fill((coords.0, coords.1 + 1), map, num);
        flood_fill((coords.0 + 1, coords.1), map, num);
        if coords.1 != 0 {
            flood_fill((coords.0, coords.1 - 1), map, num);
        }
        if coords.0 != 0 {
            flood_fill((coords.0 - 1, coords.1), map, num);
        }
    }
}

#[test]
fn test1() {
    let input: Vec<[u8; 10]> = "2199943210
3987894921
9856789892
8767896789
9899965678"
        .lines()
        .map(|l| {
            let mut row = [0; 10];
            l.chars()
                .map(|c| c as u8 - b'0')
                .enumerate()
                .for_each(|(idx, c)| row[idx] = c);
            row
        })
        .collect();
    let mut map = Map { points: input };
    assert_eq!(solve1(&map), 15);
    assert_eq!(solve2(&mut map), 1134);
}
