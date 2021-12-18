use std::{io::Read, ops::RangeInclusive};

fn main() {
    let mut input = String::new();
    input::read_file("puzzles/day17.txt")
        .read_to_string(&mut input)
        .unwrap();
    let input = parse_input(&input);

    println!("solution 1: {}", solve1(&input));
    println!("solution 2: {}", solve2(&input));
}

fn simulate(vel: (i32, i32), end: &(RangeInclusive<i32>, RangeInclusive<i32>)) -> Option<i32> {
    let (mut x, mut y) = (0, 0);
    let (mut x_vel, mut y_vel) = vel;

    let mut max_y = 0;

    while y >= *end.1.start() {
        x += x_vel;
        y += y_vel;
        max_y = max_y.max(y);

        if end.0.contains(&x) && end.1.contains(&y) {
            return Some(max_y);
        }

        x_vel = 0.max(x_vel - 1);
        y_vel -= 1;
    }

    None
}

fn solve1(target: &(RangeInclusive<i32>, RangeInclusive<i32>)) -> i32 {
    let mut max_y = 0;

    for x_vel in 0..=*target.0.end() {
        if simulate((x_vel, 0), target).is_some() {
            for y_vel in 0..target.1.start().abs() {
                if let Some(c) = simulate((x_vel, y_vel), target) {
                    max_y = max_y.max(c);
                }
            }
        }
    }

    max_y
}

fn solve2(target: &(RangeInclusive<i32>, RangeInclusive<i32>)) -> i32 {
    let mut velocities = 0;

    for x_vel in 0..=*target.0.end() {
        for y_vel in -(target.1.start().abs())..target.1.start().abs() {
            if simulate((x_vel, y_vel), target).is_some() {
                velocities += 1;
            }
        }
    }

    velocities
}

fn parse_input(s: &str) -> (RangeInclusive<i32>, RangeInclusive<i32>) {
    let x = s.find(|c| c == 'x').unwrap();
    let dots = s.find(|c| c == '.').unwrap();
    let x_start: i32 = s[x + 2..dots].parse().unwrap();
    let comma = s.find(|c| c == ',').unwrap();
    let x_end: i32 = s[dots + 2..comma].parse().unwrap();

    let y = s.find(|c| c == 'y').unwrap();
    let dots = s[dots + 2..].find(|c| c == '.').unwrap() + dots + 2;
    let y_start: i32 = s[y + 2..dots].parse().unwrap();
    let y_end: i32 = s[dots + 2..].trim_end().parse().unwrap();

    (x_start..=x_end, y_start..=y_end)
}

#[test]
fn test() {
    assert_eq!(simulate((7, 2), &(20..=30, -10..=-5)), Some(3));
    assert_eq!(simulate((6, 3), &(20..=30, -10..=-5)), Some(6));
    assert_eq!(simulate((9, 0), &(20..=30, -10..=-5)), Some(0));
    assert_eq!(simulate((6, 0), &(20..=30, -10..=-5)), Some(0));
    assert_eq!(simulate((17, -4), &(20..=30, -10..=-5)), None);

    assert_eq!(solve1(&(20..=30, -10..=-5)), 45);
    assert_eq!(solve2(&(20..=30, -10..=-5)), 112);
}
