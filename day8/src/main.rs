use std::{fmt::Write, str::FromStr};

struct Entry {
    patterns: [Vec<char>; 10],
    codes: [Vec<char>; 4],
}

impl FromStr for Entry {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const EMPTY_VEC: Vec<char> = Vec::new();
        let mut patterns = [EMPTY_VEC; 10];
        let mut codes = [EMPTY_VEC; 4];

        let mut iter = s.split(' ');
        for pat in &mut patterns {
            let mut info: Vec<char> = iter.next().unwrap().trim().chars().collect();
            info.sort_unstable();
            *pat = info;
        }
        // skip the delimiter
        iter.next();
        for code in &mut codes {
            let mut info: Vec<char> = iter.next().unwrap().trim().chars().collect();
            info.sort_unstable();
            *code = info;
        }

        patterns.sort_by(|a, b| a.len().cmp(&b.len()));

        Ok(Entry { patterns, codes })
    }
}

impl std::fmt::Debug for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("patterns: ")?;
        for pat in &self.patterns {
            for c in pat {
                f.write_char(*c)?;
            }
            f.write_str(", ")?;
        }
        f.write_str("\ncodes: ")?;
        for pat in &self.codes {
            for c in pat {
                f.write_char(*c)?;
            }
            f.write_str(", ")?;
        }
        Ok(())
    }
}

fn main() {
    let mut entries: Vec<Entry> = input::read_lines("puzzles/day8.txt");
    let solution = solve1(&entries);
    println!("solution 1: {}", solution);
    let solution = solve2(&mut entries);
    println!("solution 2: {}", solution);
}

fn solve1(entries: &[Entry]) -> usize {
    let mut digits = 0;
    for entry in entries {
        digits += entry
            .codes
            .iter()
            .filter(|c| matches!(c.len(), 2 | 3 | 4 | 7))
            .count();
    }
    digits
}

fn solve2(entries: &mut [Entry]) -> usize {
    let mut result = 0;
    for entry in entries {
        arrange_numbers(&mut entry.patterns);
        result += calc_code(entry);
    }
    result
}

fn calc_code(entry: &Entry) -> usize {
    let mut result = 0;
    for code in &entry.codes {
        result *= 10;
        let value = entry.patterns.iter().position(|pat| pat == code).unwrap();
        result += value;
    }
    result
}

// Assumes patterns and letters in them are sorted
fn arrange_numbers(patterns: &mut [Vec<char>; 10]) {
    // Put 7, 1, 4 and 8 in the correct place
    patterns.swap(1, 7);
    patterns.swap(0, 1);
    patterns.swap(2, 4);
    patterns.swap(9, 8);
    // 9 is the only 6-length that contains all of 4
    let nine = patterns
        .iter()
        .enumerate()
        .filter(|(_, pat)| pat.len() == 6)
        .find(|(_, pat)| contains_all(*pat, &patterns[4]))
        .map(|(idx, _)| idx)
        .unwrap();
    patterns.swap(nine, 9);
    // 6 is the only 6-length that doesn't contain 7
    let six = patterns
        .iter()
        .enumerate()
        .filter(|(_, pat)| pat.len() == 6)
        .find(|(_, pat)| !contains_all(*pat, &patterns[7]))
        .map(|(idx, _)| idx)
        .unwrap();
    patterns.swap(six, 6);
    // 0 is the only 6-length left
    let zero = patterns
        .iter()
        .enumerate()
        .find(|(_, pat)| (*pat != &patterns[9] && *pat != &patterns[6]))
        .map(|(idx, _)| idx)
        .unwrap();
    patterns.swap(zero, 0);
    // Correct ones by now: 0, 1, 4, 6, 7, 8, 9
    // only 2, 3 and 5 left
    // 3 is the only 5-length that contains all of 7
    let three = patterns
        .iter()
        .enumerate()
        .filter(|(_, pat)| pat.len() == 5)
        .find(|(_, pat)| contains_all(*pat, &patterns[7]))
        .map(|(idx, _)| idx)
        .unwrap();
    patterns.swap(three, 3);
    // 5 is contained within 6
    let five = patterns
        .iter()
        .enumerate()
        .filter(|(_, pat)| pat.len() == 5)
        .find(|(_, pat)| contains_all(&patterns[6], *pat))
        .map(|(idx, _)| idx)
        .unwrap();
    patterns.swap(five, 5);
    // 2 is already ordered
}

// Assumes both slices are sorted
fn contains_all<T: PartialEq>(superset: &[T], subset: &[T]) -> bool {
    if subset.len() > superset.len() {
        return false;
    }

    let mut superset_pos = 0;
    for c in subset.iter() {
        while match superset.get(superset_pos) {
            Some(cs) if cs == c => false,
            None => return false,
            Some(_) => true,
        } {
            superset_pos += 1;
        }
    }
    true
}

#[test]
fn test_short() {
    let mut entries = vec![Entry::from_str(
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe",
    )
    .unwrap()];
    let result = solve2(&mut entries);
    assert_eq!(result, 8394);
}

#[test]
fn test_long() {
    let input =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";
    let mut entries: Vec<Entry> = input.lines().map(|l| Entry::from_str(l).unwrap()).collect();
    let result = solve2(&mut entries);
    assert_eq!(result, 61229);
}

#[test]
fn test_contains_all() {
    let superset = [1, 2, 3, 4];
    let subset = [1, 2, 3];
    assert!(contains_all(&superset, &subset));
    let subset = [2, 3];
    assert!(contains_all(&superset, &subset));
    let subset = [2, 5];
    assert!(!contains_all(&superset, &subset));
    let superset = [2, 3, 4, 5];
    let subset = [1, 3, 5];
    assert!(!contains_all(&superset, &subset));
}
