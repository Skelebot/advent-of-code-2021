use std::{num::ParseIntError, str::FromStr};

enum Command {
    Forward(u32),
    Up(u32),
    Down(u32),
}

impl FromStr for Command {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let space_idx = s
            .chars()
            .enumerate()
            .find(|(_, c)| *c == ' ')
            .map(|(i, _)| i)
            .expect("no space found in command");
        let num: u32 = s[space_idx + 1..].parse()?;

        match &s[0..space_idx] {
            "forward" => Ok(Command::Forward(num)),
            "up" => Ok(Command::Up(num)),
            "down" => Ok(Command::Down(num)),
            _ => panic!("invalid command"),
        }
    }
}

fn main() {
    let input: Vec<Command> = input::read_lines("puzzles/day2.txt");

    let mut horizontal = 0;
    let mut vertical = 0;

    for cmd in &input {
        match cmd {
            Command::Forward(n) => horizontal += n,
            Command::Up(n) => vertical -= n,
            Command::Down(n) => vertical += n,
        }
    }

    println!("solution 1: {}", horizontal * vertical);

    horizontal = 0;
    vertical = 0;
    let mut aim = 0;

    for cmd in &input {
        match cmd {
            Command::Forward(n) => {
                horizontal += n;
                vertical += aim * n;
            }
            Command::Up(n) => aim -= n,
            Command::Down(n) => aim += n,
        }
    }

    println!("solution 2: {}", horizontal * vertical);
}
