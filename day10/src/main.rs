fn main() {
    let input: Vec<String> = input::read_lines("puzzles/day10.txt");
    println!("solution 1: {}", solve1(&input));
    println!("solution 2: {}", solve2(&input));
}

const fn is_opening(c: char) -> bool {
    matches!(c, '(' | '[' | '{' | '<')
}

const fn byteval(c: char) -> u8 {
    match c {
        '(' | ')' => (1),
        '[' | ']' => (2),
        '{' | '}' => (3),
        '<' | '>' => (4),
        _ => unreachable!(),
    }
}

const fn get_score(c: u8) -> usize {
    match c {
        1 => 3,
        2 => 57,
        3 => 1197,
        4 => 25137,
        _ => unreachable!(),
    }
}

fn solve1(input: &[String]) -> usize {
    let mut score: usize = 0;
    let mut stack: Vec<u8> = Vec::with_capacity(150);
    'main: for line in input {
        stack.clear();
        for cc in line.chars() {
            let c = byteval(cc);
            if is_opening(cc) {
                stack.push(c);
            } else {
                let co = stack.pop().unwrap();
                if co != c {
                    score += get_score(c);
                    continue 'main;
                }
            }
        }
    }

    score
}

fn solve2(input: &[String]) -> usize {
    let mut scores: Vec<usize> = Vec::new();
    let mut stack: Vec<u8> = Vec::with_capacity(150);
    'main: for line in input {
        let mut score: usize = 0;
        stack.clear();
        for cc in line.chars() {
            let c = byteval(cc);
            if is_opening(cc) {
                stack.push(c);
            } else {
                let co = stack.pop().unwrap();
                if co != c {
                    continue 'main;
                }
            }
        }
        while let Some(left) = stack.pop() {
            score *= 5;
            score += left as usize;
        }
        scores.push(score);
    }

    scores.sort_unstable();
    scores[scores.len() / 2]
}

#[test]
fn test1() {
    let input = vec![
        "[({(<(())[]>[[{[]{<()<>>".to_string(),
        "[(()[<>])]({[<{<<[]>>(".to_string(),
        "{([(<{}[<>[]}>{[]{[(<()>".to_string(),
        "(((({<>}<{<{<>}{[]{[]{}".to_string(),
        "[[<[([]))<([[{}[[()]]]".to_string(),
        "[{[{({}]{}}([{[{{{}}([]".to_string(),
        "{<[[]]>}<{[{[{[]{()[[[]".to_string(),
        "[<(<(<(<{}))><([]([]()".to_string(),
        "<{([([[(<>()){}]>(<<{{".to_string(),
        "<{([{{}}[<[[[<>{}]]]>[]]".to_string(),
    ];
    assert_eq!(solve1(&input), 26397);
    assert_eq!(solve2(&input), 288957);
}
