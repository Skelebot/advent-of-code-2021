use std::{ops::RangeInclusive, str::FromStr};

struct Cuboid {
    command: bool,
    x: RangeInclusive<i32>,
    y: RangeInclusive<i32>,
    z: RangeInclusive<i32>,
}

impl Cuboid {
    fn contains_point(&self, point: &(i32, i32, i32)) -> bool {
        self.x.contains(&point.0) && self.y.contains(&point.1) && self.z.contains(&point.2)
    }

    fn intersect_ranges(a: &RangeInclusive<i32>, b: &RangeInclusive<i32>) -> RangeInclusive<i32> {
        *a.start().max(b.start())..=*a.end().min(b.end())
    }

    fn intersects(&self, rhs: &Cuboid) -> bool {
        self.x.start() <= rhs.x.end()
            && self.x.end() >= rhs.x.start()
            && self.y.start() <= rhs.y.end()
            && self.y.end() >= rhs.y.start()
            && self.z.start() <= rhs.z.end()
            && self.z.end() >= rhs.z.start()
    }

    fn intersection(&self, rhs: &Cuboid) -> Option<Cuboid> {
        if !self.intersects(rhs) {
            return None;
        }
        Some(Cuboid {
            command: self.command,
            x: Self::intersect_ranges(&self.x, &rhs.x),
            y: Self::intersect_ranges(&self.y, &rhs.y),
            z: Self::intersect_ranges(&self.z, &rhs.z),
        })
    }

    fn calc_volume(&self, next: &[Cuboid]) -> usize {
        let inner = next
            .iter()
            .filter_map(|c2| c2.intersection(self))
            .collect::<Vec<_>>();
        let vsub: usize = (0..inner.len())
            .map(|i| inner[i].calc_volume(&inner[i + 1..]))
            .sum();
        self.volume() - vsub
    }

    fn volume(&self) -> usize {
        (self.x.end() - self.x.start() + 1) as usize
            * (self.y.end() - self.y.start() + 1) as usize
            * (self.z.end() - self.z.start() + 1) as usize
    }
}

impl FromStr for Cuboid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(|c| matches!(c, ' ' | ','));
        let command = match iter.next().unwrap() {
            "on" => true,
            "off" => false,
            _ => unreachable!(),
        };
        const DEF: RangeInclusive<i32> = 0..=0;
        let mut xyz: [RangeInclusive<i32>; 3] = [DEF; 3];
        for r in xyz.iter_mut() {
            let s = iter.next().unwrap();
            let dot = s.find(|c| c == '.').unwrap();
            let start = s[2..dot].parse().unwrap();
            let end = s[dot + 2..].parse().unwrap();

            *r = start..=end
        }

        let [x, y, z] = xyz;

        Ok(Cuboid { command, x, y, z })
    }
}

fn main() {
    let input: Vec<Cuboid> = input::read_lines("puzzles/day22.txt");

    println!("solution 1: {}", solve1(&input));
    println!("solution 2: {}", solve2(&input));
}

fn solve1(cuboids: &[Cuboid]) -> usize {
    let mut on = 0;

    for x in -50..=50 {
        for y in -50..=50 {
            for z in -50..=50 {
                let mut this_on = false;
                for cuboid in cuboids {
                    if cuboid.contains_point(&(x, y, z)) {
                        this_on = cuboid.command;
                    }
                }
                if this_on {
                    on += 1;
                }
            }
        }
    }
    on
}

fn solve2(cuboids: &[Cuboid]) -> usize {
    (0..cuboids.len())
        .filter(|&i| cuboids[i].command)
        .map(|i| cuboids[i].calc_volume(&cuboids[i + 1..]))
        .sum()
}
