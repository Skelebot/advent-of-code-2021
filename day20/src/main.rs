use std::io::BufRead;

fn main() {
    let mut iter = input::read_file("puzzles/day20.txt").lines();
    let rules: Vec<u8> = iter
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .map(|c| c as u8)
        .collect();

    iter.next();

    let mut image: Vec<Vec<u8>> = iter
        .map(|c| c.unwrap().chars().map(|c| c as u8).collect())
        .collect();

    step(&mut image, &rules, false);
    step(&mut image, &rules, true);

    println!("solution 1: {}", count_lit(&image));

    for i in 2..50 {
        step(&mut image, &rules, i % 2 != 0);
    }

    println!("solution 2: {}", count_lit(&image));
}

fn step(img: &mut Vec<Vec<u8>>, rules: &[u8], even: bool) {
    let mut new_img: Vec<Vec<u8>> = (0..img.len() + 2)
        .map(|_| vec![b'.'; img.len() + 2])
        .collect();

    for y in -1..img.len() as isize + 1 {
        for x in -1..img.len() as isize + 1 {
            let num = neigbor_num(img, (x, y), even);
            new_img[(y + 1) as usize][(x + 1) as usize] = rules[num];
        }
    }

    *img = new_img
}

fn neigbor_num(img: &[Vec<u8>], coords: (isize, isize), out_filled: bool) -> usize {
    let mut num: usize = if out_filled { 511 } else { 0 };
    let x_start = (coords.0 - 1).max(0) as usize;
    let x_end = (coords.0 + 1).min(img.len() as isize - 1) as usize;
    let y_start = (coords.1 - 1).max(0) as usize;
    let y_end = (coords.1 + 1).min(img.len() as isize - 1) as usize;

    #[allow(clippy::needless_range_loop)]
    for y in y_start..=y_end {
        for x in x_start..=x_end {
            let rel_x = x as isize - coords.0 + 1;
            let rel_y = y as isize - coords.1 + 1;
            let weight = (rel_y as usize * 3) + rel_x as usize;
            if img[y][x] == b'#' {
                num |= 1 << weight;
            } else {
                num &= !(1 << weight);
            }
        }
    }
    num = num.reverse_bits();
    num >>= usize::BITS - 9;

    num
}

fn count_lit(img: &[Vec<u8>]) -> usize {
    img.iter()
        .map(|c| c.iter().filter(|c| **c == b'#').count())
        .sum()
}

#[test]
fn test_num() {
    let img = vec![
        vec![b'.'; 3],
        vec![b'#', b'.', b'.'],
        vec![b'.', b'#', b'.'],
    ];
    assert_eq!(neigbor_num(&img, (1, 1), false), 34);
    let img = vec![vec![b'#']];
    assert_eq!(neigbor_num(&img, (-1, -1), false), 1);
    assert_eq!(neigbor_num(&img, (1, 1), false), 0b1_0000_0000);
}

#[test]
fn test_step() {
    let rules: Vec<u8> = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#"
    .chars().map(|c| c as u8).collect();
    let mut img: Vec<Vec<u8>> = "#..#.
#....
##..#
..#..
..###"
        .lines()
        .map(|c| c.chars().map(|c| c as u8).collect())
        .collect();

    step(&mut img, &rules, false);
    step(&mut img, &rules, false);

    assert_eq!(count_lit(&img), 35);

    for _ in 2..50 {
        step(&mut img, &rules, false);
    }
    assert_eq!(count_lit(&img), 3351);
}
