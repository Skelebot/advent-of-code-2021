use std::{collections::HashMap, io::BufRead};

fn step(template: &mut HashMap<(char, char), usize>, rules: &HashMap<(char, char), char>) {
    let mut new = HashMap::new();
    for (pair, counter) in template.iter() {
        let out = rules[pair];
        *new.entry((pair.0, out)).or_insert(0) += counter;
        *new.entry((out, pair.1)).or_insert(0) += counter;
    }
    *template = new;
}

fn score(last: char, template: &HashMap<(char, char), usize>) -> usize {
    let mut counts = HashMap::new();
    for (pair, count) in template.iter() {
        *counts.entry(pair.0).or_insert(0) += count;
    }
    *counts.entry(last).or_insert(0) += 1;

    let max = counts.values().max().unwrap();
    let min = counts.values().min().unwrap();

    max - min
}

fn main() {
    let mut lines = input::read_file("puzzles/day14.txt").lines();
    let template: Vec<char> = lines.next().unwrap().unwrap().trim().chars().collect();
    let last = *template.last().unwrap();

    lines.next();

    let rules: HashMap<(char, char), char> = lines
        .map(|c| {
            let s = c.unwrap();
            let mut c = s.chars();
            ((c.next().unwrap(), c.next().unwrap()), c.last().unwrap())
        })
        .collect();

    let mut template: HashMap<(char, char), usize> =
        template.windows(2).map(|w| ((w[0], w[1]), 1)).collect();

    for _ in 0..10 {
        step(&mut template, &rules);
    }
    println!("solution 1: {}", score(last, &template));

    for _ in 10..40 {
        step(&mut template, &rules);
    }
    println!("solution 2: {}", score(last, &template));
}

#[test]
fn test_small() {
    let template = vec!['N', 'N', 'C', 'B'];
    let last = *template.last().unwrap();
    let mut template: HashMap<(char, char), usize> =
        template.windows(2).map(|w| ((w[0], w[1]), 1)).collect();
    let rules = "CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";
    let rules: HashMap<(char, char), char> = rules
        .lines()
        .map(|c| {
            let mut c = c.chars();
            ((c.next().unwrap(), c.next().unwrap()), c.last().unwrap())
        })
        .collect();

    for _ in 0..10 {
        step(&mut template, &rules);
    }
    assert_eq!(score(last, &template), 1588);

    for _ in 10..40 {
        step(&mut template, &rules);
    }
    assert_eq!(score(last, &template), 2188189693529);
}
