use std::str::FromStr;

#[derive(Clone, Copy)]
struct Num {
    inner: u8,
    level: u8,
}

#[derive(Clone)]
struct Number(Vec<Num>);

impl FromStr for Number {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut numbers = Vec::new();
        let mut level = 0;

        for c in s.chars() {
            match c {
                '0'..='9' => numbers.push(Num {
                    inner: c as u8 - b'0',
                    level: level - 1,
                }),
                '[' => level += 1,
                ']' => level -= 1,
                _ => (),
            }
        }
        Ok(Number(numbers))
    }
}

impl std::ops::Deref for Number {
    type Target = Vec<Num>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Number {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Number {
    fn reduce(&mut self) {
        loop {
            if let Some(i) = self.iter().position(|n| n.level == 4) {
                {
                    let left = self.get(i).unwrap().inner;
                    let right = self.get(i + 1).unwrap().inner;
                    if i != 0 {
                        self[i - 1].inner += left;
                    }
                    if let Some(n) = self.get_mut(i + 2) {
                        n.inner += right;
                    }
                }
                self.remove(i);
                self[i] = Num { inner: 0, level: 3 };
                continue;
            } else if let Some(i) = self.iter().position(|n| n.inner >= 10) {
                let num = *self.get(i).unwrap();
                let left = num.inner / 2;
                let right = (num.inner + 1) / 2;
                self[i] = Num {
                    inner: left,
                    level: num.level + 1,
                };
                self.insert(
                    i + 1,
                    Num {
                        inner: right,
                        level: num.level + 1,
                    },
                );
                continue;
            } else {
                break;
            }
        }
    }

    fn magnitude(&self) -> usize {
        let mut to_fold: Vec<_> = self.iter().map(|n| (n.inner as usize, n.level)).collect();
        while to_fold.len() > 1 {
            let level = to_fold
                .windows(2)
                .find(|w| w[0].1 == w[1].1)
                .map(|w| w[0].1)
                .unwrap();

            let i = to_fold.iter().position(|n| n.1 == level).unwrap();
            let new = {
                let left = to_fold.get(i).unwrap();
                let right = to_fold.get(i + 1).unwrap();
                let level = if left.1 != 0 { left.1 - 1 } else { 0 };
                let inner = (3 * left.0) + (2 * right.0);
                (inner, level)
            };
            to_fold.remove(i);
            to_fold[i] = new;
        }

        to_fold[0].0
    }

    fn add(&self, rhs: &Self) -> Self {
        let mut result = self.clone();
        result.extend(rhs.iter());
        result.iter_mut().for_each(|n| n.level += 1);
        result.reduce();
        result
    }
}

fn main() {
    let nums: Vec<Number> = input::read_lines("puzzles/day18.txt");

    let mut num = nums[0].clone();
    for n in nums[1..].iter() {
        num = num.add(n);
    }
    println!("solution 1: {}", num.magnitude());

    let mut max_magnitude = 0;
    for a in 0..nums.len() {
        for b in 0..nums.len() {
            if a == b {
                continue;
            }
            let mag = nums[a].add(&nums[b]).magnitude();
            let mag_rev = nums[b].add(&nums[a]).magnitude();
            max_magnitude = max_magnitude.max(mag).max(mag_rev);
        }
    }
    println!("solution 2: {}", max_magnitude);
}
