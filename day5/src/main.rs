use std::{mem::swap, str::FromStr};

#[derive(Debug, Default, PartialEq, Clone, Copy)]
struct Line {
    start: (usize, usize),
    end: (usize, usize),
}

impl Line {
    #[cfg(test)]
    pub fn new(x1: usize, y1: usize, x2: usize, y2: usize) -> Self {
        let mut line = Line {
            start: (x1, y1),
            end: (x2, y2),
        };
        line.normalize();
        line
    }
    pub fn normalize(&mut self) {
        if self.start > self.end {
            swap(&mut self.start, &mut self.end);
        }
    }
    pub const fn is_hv(&self) -> bool {
        self.is_horiz() || self.is_vert()
    }
    pub const fn is_vert(&self) -> bool {
        self.start.0 == self.end.0
    }
    pub const fn is_horiz(&self) -> bool {
        self.start.1 == self.end.1
    }
    pub fn covers(&self, point: (usize, usize)) -> bool {
        if self.is_hv() {
            self.start.0 <= point.0
                && point.0 <= self.end.0
                && self.start.1 <= point.1
                && point.1 <= self.end.1
        } else {
            self.start <= point && point <= self.end && check(self.start, point, self.end)
        }
    }
}

fn check(a: (usize, usize), b: (usize, usize), c: (usize, usize)) -> bool {
    let (ax, ay) = (a.0 as i32, a.1 as i32);
    let (bx, by) = (b.0 as i32, b.1 as i32);
    let (cx, cy) = (c.0 as i32, c.1 as i32);
    // cross product
    ((cy - ay) * (bx - ax) - (cx - ax) * (by - ay)) == 0
    // dot product
    && ((cx - ax) * (bx - ax) + (cy - ay) * (by - ay)) >= 0
}

impl FromStr for Line {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut line = Line::default();

        let comma = s.find(',').unwrap();
        line.start.0 = s[0..comma].parse().map_err(|_| "Failed to parse number")?;

        let space = s.find(' ').unwrap();
        line.start.1 = s[comma + 1..space]
            .parse()
            .map_err(|_| "Failed to parse number")?;

        let next_comma = s[space..].find(',').unwrap() + space;
        line.end.0 = s[space + 4..next_comma]
            .parse()
            .map_err(|_| "Failed to parse number")?;
        line.end.1 = s[next_comma + 1..]
            .trim_end()
            .parse()
            .map_err(|_| "Failed to parse number")?;

        line.normalize();
        Ok(line)
    }
}

fn main() {
    let mut lines: Vec<Line> = input::read_lines("puzzles/day5.txt");
    lines.iter_mut().for_each(|l| l.normalize());

    let mut points_covered = 0;
    for x in 0..1000 {
        for y in 0..1000 {
            let mut covered_by = 0;
            for line in lines.iter().filter(|l| l.is_hv()) {
                if line.covers((x, y)) {
                    covered_by += 1;
                    if covered_by == 2 {
                        points_covered += 1;
                        break;
                    }
                }
            }
        }
    }
    println!("solution 1: {}", points_covered);
    let mut points_covered = 0;
    for x in 0..1000 {
        for y in 0..1000 {
            let mut covered_by = 0;
            for line in lines.iter() {
                if line.covers((x, y)) {
                    covered_by += 1;
                    if covered_by == 2 {
                        points_covered += 1;
                        break;
                    }
                }
            }
        }
    }
    println!("solution 2: {}", points_covered);
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_line() {
        let line = Line::from_str("301,93 -> 301,238\n");
        assert!(line.is_ok());
        let line = line.unwrap();
        assert_eq!(line, Line::new(301, 93, 301, 238));
        assert!(line.covers((301, 100)));
        let line = Line::new(8, 2, 8, 10);
        assert!(line.is_vert());
        assert!(line.covers((8, 2)));

        let mut line = Line::new(4, 1, 2, 3);
        line.normalize();
        assert_eq!(line.start, (2, 3));
        assert!(line.covers((3, 2)));
        assert!(!line.covers((1, 4)));

        let mut line = Line::new(4, 4, 2, 2);
        line.normalize();
        assert_eq!(line.start, (2, 2));
        assert!(line.covers((3, 3)));
        assert!(!line.covers((1, 1)));
        assert!(!line.covers((5, 5)));

        let lines = vec![
            Line::new(0, 9, 5, 9),
            Line::new(8, 0, 0, 8),
            Line::new(9, 4, 3, 4),
            Line::new(2, 2, 2, 1),
            Line::new(7, 0, 7, 4),
            Line::new(6, 4, 2, 0),
            Line::new(0, 9, 2, 9),
            Line::new(3, 4, 1, 4),
            Line::new(0, 0, 8, 8),
            Line::new(5, 5, 8, 2),
        ];
        assert_eq!(plot(lines), 12);
    }

    fn plot(lines: Vec<Line>) -> usize {
        let mut points_covered = 0;
        for r in 0..10 {
            for c in 0..10 {
                let mut covered_by = 0;
                //for line in lines.iter().filter(|c| c.is_hv()) {
                    for line in lines.iter() {
                    if line.covers((c, r)) {
                        covered_by += 1;
                        if covered_by == 2 {
                            points_covered += 1;
                            break;
                        }
                    }
                }
                if covered_by > 0 {
                    print!("{}", covered_by)
                } else {
                    print!(".")
                }
            }
            println!();
        }
        points_covered
    }
}
