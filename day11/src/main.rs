use std::io::BufRead;

struct OctopiField<const N: usize> {
    field: [[u8; N]; N],
    flashes: u32,
}

impl<const N: usize> OctopiField<N> {
    pub fn new<I: ToString>(iter: impl Iterator<Item = I>) -> Self {
        let mut field = [[0; N]; N];
        for (y, line) in iter.enumerate() {
            for (x, c) in line.to_string().chars().enumerate() {
                field[y][x] = c as u8 - b'0';
            }
        }
        OctopiField { field, flashes: 0 }
    }
    pub fn step(&mut self) -> u32 {
        let flashes_before = self.flashes;
        for y in 0..N {
            for x in 0..N {
                if self.field[y][x] == 0 {
                    self.field[y][x] = u8::MAX
                }
            }
        }
        for y in 0..N {
            for x in 0..N {
                let val = &mut self.field[y][x];
                if *val == 0 {
                    continue;
                }
                if *val == u8::MAX {
                    *val = 1;
                } else {
                    *val += 1;
                }
                if *val > 9 {
                    *val = 0;
                    let mut flashed = [[false; N]; N];
                    self.flash(x, y, &mut flashed);
                }
            }
        }
        self.flashes - flashes_before
    }
    fn flash(&mut self, x: usize, y: usize, flashed: &mut [[bool; N]; N]) {
        self.flashes += 1;
        flashed[y][x] = true;
        let xs = if x == 0 { 0 } else { x - 1 };
        let ys = if y == 0 { 0 } else { y - 1 };
        for y in ys..N.min(y + 2) {
            for x in xs..N.min(x + 2) {
                let val = &mut self.field[y][x];
                if *val == 0 {
                    continue;
                }
                if !flashed[y][x] {
                    if *val == u8::MAX {
                        *val = 1;
                    } else {
                        *val += 1;
                    }
                    if *val > 9 {
                        *val = 0;
                        self.flash(x, y, flashed)
                    }
                }
            }
        }
    }
}

fn main() {
    let mut input = OctopiField::<10>::new(
        input::read_file("puzzles/day11.txt")
            .lines()
            .map(|l| l.unwrap()),
    );
    for _ in 0..100 {
        input.step();
    }
    println!("solution 1: {}", input.flashes);
    let mut i = 101;
    while input.step() != 100 {
        i += 1;
    }
    println!("solution 2: {}", i);
}

#[test]
fn test_flashes() {
    let input = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";
    let mut input = OctopiField::<10>::new(input.lines());
    for _ in 0..10 {
        input.step();
    }
    assert_eq!(input.flashes, 204);
    let mut i = 11;
    while input.step() != 100 {
        i += 1;
    }
    assert_eq!(i, 195);
}

#[test]
fn test_simple() {
    let input = "11111
19991
19191
19991
11111";
    let mut input = OctopiField::<5>::new(input.lines());
    assert_eq!(input.step(), 9);
    assert_eq!(input.step(), 0);
}
