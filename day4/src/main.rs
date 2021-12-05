use std::io::{BufRead, Lines};

#[derive(Debug)]
pub struct Bingo {
    pub inner: [[u8; 5]; 5],
}
impl Bingo {
    pub fn from_lines(iter: &mut Lines<impl BufRead>) -> Self {
        let mut bingo = Bingo { inner: [[0; 5]; 5] };
        for row in 0..5 {
            iter.next()
                .unwrap()
                .unwrap()
                .split_whitespace()
                .map(|s| s.parse::<u8>())
                .enumerate()
                .for_each(|(i, n)| {
                    bingo.inner[row][i] = n.unwrap();
                });
        }
        bingo
    }

    pub fn mark(&mut self, num: u8) {
        for x in 0..5 {
            for y in 0..5 {
                if self.inner[x][y] == num {
                    self.inner[x][y] |= 128;
                    return;
                }
            }
        }
    }

    pub fn is_marked(&self, loc: (usize, usize)) -> bool {
        self.inner[loc.0][loc.1] & 128 > 0
    }

    pub fn wins(&self) -> bool {
        for row in 0..5 {
            if self.inner[row].iter().all(|n| n & 128 > 0) {
                return true;
            }
        }
        for col in 0..5 {
            if (0..5).all(|row| self.is_marked((row, col))) {
                return true;
            }
        }

        false
    }

    pub fn score(&self) -> usize {
        let mut sum: usize = 0;
        for row in 0..5 {
            sum += self.inner[row]
                .iter()
                .filter(|n| **n & 128 == 0)
                .map(|c| *c as usize)
                .sum::<usize>();
        }
        sum
    }
}

fn read_bingos(mut reader: impl BufRead) -> (Vec<u8>, Vec<Bingo>) {
    let mut buf = String::new();
    let numbers: Vec<u8> = {
        reader.read_line(&mut buf).unwrap();
        buf.split(',').map(|s| s.trim().parse().unwrap()).collect()
    };
    let mut bingos: Vec<Bingo> = Vec::new();
    let mut lines = reader.lines();
    lines.next();
    loop {
        bingos.push(Bingo::from_lines(&mut lines));
        if lines.next().is_none() {
            break;
        }
    }

    (numbers, bingos)
}

fn main() {
    let reader = input::read_file("puzzles/day4.txt");
    let (numbers, mut bingos) = read_bingos(reader);
    let mut last_won = None;
    for num in numbers {
        for (idx, bingo) in bingos.iter_mut().enumerate() {
            if !bingo.wins() {
                bingo.mark(num);
                if bingo.wins() {
                    if last_won.is_none() {
                        println!("solution 1: {}", bingo.score() * num as usize);
                    }
                    last_won = Some((idx, num));
                }
            }
        }
    }
    if let Some((idx, num)) = last_won {
        println!("solution 2: {}", bingos[idx].score() * num as usize);
    }
}

#[test]
fn test_bingo() {
    let mut b = Bingo {
        inner: [
            [0, 1, 0, 0, 0],
            [0, 2, 0, 10, 0],
            [0, 3, 0, 0, 0],
            [0, 4, 0, 0, 0],
            [18, 5, 0, 0, 0],
        ],
    };
    for i in 1..=5 {
        b.mark(i);
    }
    assert!(b.wins());
    assert_eq!(b.score(), 28)
}
