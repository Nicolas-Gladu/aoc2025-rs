use advent_of_code::utils::parse_u64;
use std::fs::File;
use std::io::Read;
use std::ops::RangeInclusive;
use std::time::Instant;

fn parse(input: &[u8]) -> (Vec<RangeInclusive<u64>>, Vec<u64>) {
    let mut ranges = Vec::with_capacity(200);  // Reasonable estimate
    let mut numbers = Vec::with_capacity(1200);

    let mut lines = input.split(|c| *c == b'\n');
    for line in lines.by_ref() {
        if line.is_empty() || line == b"\r" {
            break; // finished with the ranges
        }
        if let Some(pos) = line.iter().position(|&b| b == b'-') {
            let start = parse_u64(&line[..pos]);
            let end = parse_u64(&line[pos + 1..]);
            ranges.push(start..=end);
        }
    }


    for line in lines {
        if !line.is_empty() && line != b"\r" {
            numbers.push(parse_u64(line));
        }
    }


    (ranges, numbers)
}

fn solve_part1(ranges: &[RangeInclusive<u64>], numbers: &[u64]) -> u32 {
    let now = Instant::now();
    let answer = numbers.iter().filter(|&x| ranges.iter().any(|range| range.contains(x))).count() as u32;
    let elapsed = now.elapsed();
    println!("elapsed: {:?}", elapsed);
    answer
}

fn solve_part2(ranges: &mut [RangeInclusive<u64>]) -> u64 {
    let now = Instant::now();
    ranges.sort_by_key(|x| *x.start());

    let mut last: Option<RangeInclusive<u64>> = None;
    let mut count = 0;
    for range in ranges {
        match last {
            Some(ref mut last_val) => {
                if range.start() <= &(last_val.end() + 1) {
                    let start = last_val.start();
                    if range.end() > last_val.end() {
                        last = Some(*start..=*range.end());
                    }
                } else {
                    count += last_val.end() - last_val.start() + 1;
                    last = Some(range.clone());
                }
            }
            None => {
                last = Some(range.clone());
                continue;
            }
        }
    }

    if let Some(last_val) = last {
        count += last_val.end() - last_val.start() + 1;
    }
    let elapsed = now.elapsed();
    println!("took: {:?}", elapsed);
    count as u64
}

fn main() {
    let mut buf = Vec::new();
    File::open("inputs/day05.txt")
        .unwrap()
        .read_to_end(&mut buf)
        .unwrap();

    let s = String::from_utf8(buf).unwrap();
    let s = s.replace("\r\n", "\n");
    let buf = s.into_bytes();

    let (mut ranges, numbers) = parse(&buf);

    println!("Part 1: {}", solve_part1(&ranges, &numbers));
    println!("Part 2: {}", solve_part2(&mut ranges));
}

#[cfg(test)]
mod tests {
    use crate::{parse, solve_part1, solve_part2};

    const INPUT: &'static [u8] = b"3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32";

    #[test]
    fn test_parse() {
        let (ranges, vals) = parse(INPUT);
        assert_eq!(ranges, vec![3..=5, 10..=14, 16..=20, 12..=18]);
        assert_eq!(vals, vec![1, 5, 8, 11, 17, 32]);
    }

    #[test]
    fn test_pt1() {
        let (ranges, vals) = parse(INPUT);
        let result = solve_part1(&ranges, &vals);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_pt2() {
        let (mut ranges, _) = parse(INPUT);
        let result = solve_part2(&mut ranges);
        assert_eq!(result, 14);
    }
}