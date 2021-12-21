use std::io::BufRead;

fn main() {
    let input: Vec<u32> = input::read_file("puzzles/day21.txt")
        .lines()
        .map(|l| {
            l.unwrap()
                .trim_end()
                .split(' ')
                .last()
                .unwrap()
                .parse()
                .unwrap()
        })
        .collect();

    println!("solution 1: {}", solve1((input[0], input[1])));
    println!("solution 2: {}", solve2((input[0], input[1])));
}

fn solve1(start: (u32, u32)) -> usize {
    let mut scores = (0, 0);
    let mut progress = start;
    let mut rolls = 0;

    loop {
        progress.0 = move_by(progress.0, 6 + (rolls * 3));
        rolls += 3;
        scores.0 += progress.0;
        if scores.0 >= 1000 {
            break;
        }

        progress.1 = move_by(progress.1, 6 + (rolls * 3));
        rolls += 3;
        scores.1 += progress.1;
        if scores.1 >= 1000 {
            break;
        }
    }

    scores.0.min(scores.1) as usize * rolls as usize
}

fn solve2(start: (u32, u32)) -> usize {
    let won = simulate_universe(start, (0, 0), 1);
    won.0.max(won.1)
}

const PROB: [usize; 7] = [1, 3, 6, 7, 6, 3, 1];

fn simulate_universe(start: (u32, u32), scores: (u32, u32), num: usize) -> (usize, usize) {
    let mut won = (0, 0);

    for i in 0..=6 {
        let pos = move_by(start.0, i + 3);
        let score = scores.0 + pos;
        if score >= 21 {
            won.0 += num * PROB[i as usize]
        } else {
            let result =
                simulate_universe((start.1, pos), (scores.1, score), num * PROB[i as usize]);
            won.0 += result.1;
            won.1 += result.0;
        }
    }

    won
}

const fn move_by(pos: u32, num: u32) -> u32 {
    let out = (pos + num) % 10;
    if out == 0 {
        10
    } else {
        out
    }
}

#[test]
fn test1() {
    assert_eq!(solve1((4, 8)), 739785);
    assert_eq!(solve2((4, 8)), 444356092776315);
}
