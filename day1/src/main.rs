fn main() {
    let input: Vec<u32> = input::read_lines("puzzles/day1.txt");

    let mut increased = 0;

    let mut last = input[0];

    for num in &input[1..] {
        if *num > last {
            increased += 1;
        }
        last = *num;
    }

    println!("solution 1: {}", increased);

    increased = 0;
    last = input[0..3].iter().sum();

    for window in input.windows(3).skip(1) {
        let sum = window.iter().sum();
        if sum > last {
            increased += 1;
        }
        last = sum;
    }

    println!("solution 2: {}", increased);
}
