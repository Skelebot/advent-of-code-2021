use std::{collections::HashSet, str::FromStr};

#[derive(Hash, PartialEq, Eq)]
struct Point(u32, u32);
impl FromStr for Point {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut nums = s.split(',');
        Ok(Point(
            nums.next().unwrap().parse().unwrap(),
            nums.next().unwrap().parse().unwrap(),
        ))
    }
}

enum Fold {
    X(u32),
    Y(u32),
}
impl FromStr for Fold {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let spec = s.split(' ').nth(2).unwrap();
        let equals = spec.find(|c| c == '=').unwrap();
        match &spec[..equals] {
            "x" => Ok(Fold::X(spec[equals + 1..].trim().parse().unwrap())),
            "y" => Ok(Fold::Y(spec[equals + 1..].trim().parse().unwrap())),
            _ => unreachable!(),
        }
    }
}

fn fold(points: &mut HashSet<Point>, fold: &Fold) {
    let mut folded = HashSet::with_capacity(points.len());
    for point in points.iter() {
        let mut x = point.0;
        let mut y = point.1;
        match fold {
            Fold::X(n) if x > *n => x = (2 * n) - x,
            Fold::Y(n) if y > *n => y = (2 * n) - y,
            _ => (),
        }
        folded.insert(Point(x, y));
    }
    *points = folded;
}

fn plot(points: &HashSet<Point>) {
    let max_x = points.iter().map(|c| c.0).max().unwrap();
    let max_y = points.iter().map(|c| c.1).max().unwrap();
    for y in 0..=max_y {
        if points.iter().any(|p| p.1 == y) {
            let mut ps: Vec<&Point> = points.iter().filter(|p| p.1 == y).collect();
            ps.sort_by(|a, b| a.0.cmp(&b.0));
            let mut last_x = 0;
            for p in ps {
                for _ in last_x..p.0 {
                    print!(" ");
                }
                print!("#");
                last_x = p.0 + 1;
            }
        } else {
            for _ in 0..=max_x {
                print!(".");
            }
        }
        println!();
    }
}

fn main() {
    let (mut points, folds): (HashSet<Point>, Vec<Fold>) = input::read_split("puzzles/day13.txt");
    let mut folds = folds.into_iter();

    fold(&mut points, &folds.next().unwrap());
    println!("solution 1: {}", points.len());

    for f in folds {
        fold(&mut points, &f);
    }

    println!("solution 2: ");
    plot(&points);
}
