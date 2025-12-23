use advent_of_code::utils::parse_u64;
use std::fs::File;
use std::io::Read;
use std::time::Instant;

struct NumBuilder {
    current_num: u64,
    current_mul: u64,
}

impl NumBuilder {
    pub fn new() -> Self {
        Self {
            current_num: 0,
            current_mul: 1,
        }
    }

    pub fn add(&mut self, num: u8) {
        let num = (num - 48) as u64;
        self.current_num += num * self.current_mul;
        self.current_mul *= 10;
    }

    pub fn take_val(self) -> u64 {
        self.current_num
    }
}

enum Operation {
    Mul(u64),
    Add(u64),
}

impl Operation {
    pub fn from_str(s: &str) -> Operation {
        match s {
            "*" => Operation::Mul(1),
            "+" => Operation::Add(0),
            _ => unreachable!(),
        }
    }

    pub fn apply(&mut self, number: u64) {
        match self {
            Operation::Mul(val) => *val *= number,
            Operation::Add(val) => *val += number,
        }
    }
}

impl std::iter::Sum<Operation> for u64 {
    fn sum<I: Iterator<Item = Operation>>(iter: I) -> u64 {
        iter.fold(0, |acc, op| acc + op)
    }
}

impl std::ops::Add<Operation> for u64 {
    type Output = u64;

    fn add(self, rhs: Operation) -> Self::Output {
        match rhs {
            Operation::Mul(v) => self + v,
            Operation::Add(v) => self + v,
        }
    }
}

fn solve_part1(input: &str) -> u64 {
    let now = Instant::now();
    let mut iter = input.lines().rev();
    let mut symbols: Vec<Operation> = iter
        .next()
        .unwrap()
        .split_whitespace()
        .map(Operation::from_str)
        .collect();
    for line in iter {
        line.split_whitespace()
            .map(|num| parse_u64(num.as_bytes()))
            .enumerate()
            .for_each(|(i, num)| {
                symbols[i].apply(num);
            });
    }
    let res = symbols.into_iter().sum();
    let elapsed = now.elapsed();
    println!("took: {:?}", elapsed);
    res
}

fn solve_part2(input: &str) -> u64 {
    let now = Instant::now();
    let lines_size = input.lines().next().unwrap().as_bytes().len();
    let mut is_mul = false;
    let mut total = 0;
    let mut operation_total = 0; // 1 when multipliying since if we use 0 it stay at 0
    for col in 0..lines_size {
        let mut num = NumBuilder::new();

        for line in input.lines().rev() {
            let c = line.as_bytes()[col];
            match c {
                32 => continue,
                42 => {
                    total += operation_total;
                    is_mul = true;
                    operation_total = 1;
                }
                43 => {
                    total += operation_total;
                    is_mul = false;
                    operation_total = 0;
                }
                48..=57 => num.add(c),
                _ => unreachable!(),
            }
        }
        let num = num.take_val();
        if num == 0 {
            continue;
        }
        if is_mul {
            operation_total *= num;
        } else {
            operation_total += num;
        }
    }
    total += operation_total;
    let elapsed = now.elapsed();
    println!("took: {:?}", elapsed);
    total
}

fn main() {
    let mut buf = Vec::new();
    File::open("inputs/day06.txt")
        .unwrap()
        .read_to_end(&mut buf)
        .unwrap();

    let s = String::from_utf8(buf).unwrap();
    let s = s.replace("\r\n", "\n");

    println!("Part 1: {}", solve_part1(&s));
    println!("Part 2: {}", solve_part2(&s));
}

#[cfg(test)]
mod tests {
    use crate::{solve_part1, solve_part2};

    const INPUT: &'static str =
        "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ";
    #[test]
    fn test_solve_part1() {
        let answer = solve_part1(INPUT);
        assert_eq!(answer, 4277556);
    }

    #[test]
    fn test_solve_part2() {
        let answer = solve_part2(INPUT);
        assert_eq!(answer, 3263827);
    }
}
