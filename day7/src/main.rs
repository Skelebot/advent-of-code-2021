fn main() {
    let input: Vec<u32> = input::read_line_split("puzzles/day7.txt", ",");
    println!("solution 1: {}", solve1(&input));
    println!("solution 2: {}", solve2(&input));
}

fn solve1(input: &[u32]) -> usize {
    let max_val = *input.iter().max().unwrap();
    let mut lowest = usize::MAX;

    for target in 0..max_val {
        let mut cost = 0;
        for pos in input {
            cost += (*pos as i32 - target as i32).abs() as usize;
        }
        lowest = cost.min(lowest);
    }
    lowest
}

fn solve2(input: &[u32]) -> usize {
    let max_val = *input.iter().max().unwrap();
    let mut lowest = usize::MAX;

    for target in 0..max_val {
        let mut cost = 0;
        for pos in input {
            let dist = (*pos as i32 - target as i32).abs() as usize;
            cost += ((1 + dist) * dist) / 2;
        }
        lowest = cost.min(lowest);
    }
    lowest
}
