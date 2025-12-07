use std::fs::File;
use std::io::Read;
use std::time::Instant;

#[inline(always)]
fn parse_u16(bytes: &[u8]) -> u16 {
    let mut n = 0u16;
    for b in bytes {
        n = n * 10 + (b - b'0') as u16;
    }
    n
}

#[derive(Eq, PartialEq)]
enum Command {
    Left(u16),
    Right(u16),
}

impl TryFrom<&[u8]> for Command {
    type Error = ();

    fn try_from(mut line: &[u8]) -> Result<Command, Self::Error> {
        if line.ends_with(b"\r") {
            line = &line[..line.len() - 1];
        }
        if line.len() < 2 {
            return Err(());
        }
        let cmd = line[0];
        let num = parse_u16(&line[1..]);
        match cmd {
            b'L' => Ok(Command::Left(num)),
            b'R' => Ok(Command::Right(num)),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Dial {
    dial: u8,
    count_roll: bool,
}

impl Dial {
    const NB_CLICK: u8 = 100;

    #[inline(always)]
    pub fn new(count_roll: bool) -> Self {
        Self {
            dial: 50,
            count_roll,
        }
    }

    #[inline(always)]
    fn go_left(&mut self, num: u16) -> Option<u16> {
        self.rotate(num, true)
    }

    #[inline(always)]
    fn go_right(&mut self, num: u16) -> Option<u16> {
        self.rotate(num, false)
    }

    #[inline(always)]
    fn rotate(&mut self, step: u16, left: bool) -> Option<u16> {
        let clicks = Self::NB_CLICK as u16;
        let mut rolls = (step as u16) / clicks;
        let step = (step as u16 % clicks) as u8;

        let old = self.dial;
        self.dial = if left {
            (self.dial + Self::NB_CLICK - step) % Self::NB_CLICK
        } else {
            (self.dial + step) % Self::NB_CLICK
        };

        let crossed_zero = (left && old < self.dial && old != 0)
            || (!left && old > self.dial && old != 0)
            || self.dial == 0;

        if self.count_roll {
            if crossed_zero {
                rolls += 1;
            }
            Some(rolls)
        } else {
            (self.dial == 0).then_some(rolls)
        }
    }
}

fn solve_part1(input: &[u8]) -> u16 {
    let mut dial = Dial::new(false);
    let now = Instant::now();
    let answer = input
        .split(|b| *b == b'\n')
        .filter_map(|line| Command::try_from(line).ok())
        .filter_map(|cmd| match cmd {
            Command::Left(num) => dial.go_left(num),
            Command::Right(num) => dial.go_right(num),
        })
        .count() as u16;

    let elapsed = now.elapsed();
    println!("took: {:?}", elapsed);
    answer
}

fn solve_part2(input: &[u8]) -> u16 {
    let mut dial = Dial::new(true);
    let now = Instant::now();
    let answer = input
        .split(|b| *b == b'\n')
        .filter_map(|line| Command::try_from(line).ok())
        .filter_map(|cmd| match cmd {
            Command::Left(num) => dial.go_left(num),
            Command::Right(num) => dial.go_right(num),
        })
        .sum::<u16>();

    let elapsed = now.elapsed();
    println!("took: {:?}", elapsed);
    answer
}

fn main() {
    let mut buf = Vec::new();
    File::open("inputs/day01.txt")
        .unwrap()
        .read_to_end(&mut buf)
        .unwrap();

    println!("Part 1: {}", solve_part1(&buf));
    println!("Part 2: {}", solve_part2(&buf));
}

#[cfg(test)]
mod tests {
    use crate::{solve_part1, solve_part2};

    #[test]
    fn test() {
        let value = [
            "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82",
        ]
        .join("\n");
        let buf = value.as_bytes();
        let res1 = solve_part1(&buf);
        let res2 = solve_part2(&buf);
        assert_eq!(res1, 3);
        assert_eq!(res2, 6);
    }
}
