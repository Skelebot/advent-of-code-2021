fn main() {
    let input: Vec<usize> = input::read_line_split("puzzles/day6.txt", ",");

    println!("solution 1: {}", solve(&input, 80));
    println!("solution 2: {}", solve(&input, 256));
}

fn solve(start: &[usize], days: u32) -> usize {
    let mut fish_spawners = [0; 9];

    for fish in start {
        fish_spawners[*fish] += 1;
    }

    for _ in 1..=days {
        fish_spawners.rotate_left(1);
        fish_spawners[6] += fish_spawners[8];
    }

    fish_spawners.iter().sum()
}

#[test]
fn test() {
    let days = &[3, 4, 3, 1, 2];
    assert_eq!(solve(days, 18), 26);
    assert_eq!(solve(days, 80), 5934);
    assert_eq!(solve(days, 256), 26984457539);
}
