use std::{fs::File, io::Read, ops::RangeInclusive, time::Instant};

#[derive(Clone)]
struct Ranges(Vec<RangeInclusive<u64>>);

impl Ranges {
    pub fn new(data: &str) -> Self {
        let mut ranges = Vec::new();
        for range_data in data.split(|x| x == ',') {
            if let Some((min, max)) = range_data.split_once(|x| x == '-') {
                ranges.push(
                    u64::from_str_radix(min, 10).unwrap()..=u64::from_str_radix(max, 10).unwrap(),
                );
            }
        }
        return Self(ranges);
    }
}

fn solve_part1(ranges: Ranges) -> u64 {
    let now = Instant::now();
    let mut buf = [0u8; 20]; // stack buffer, reused
    let mut acc = 0;
    for range in ranges.0 {
        for num in range {
            let num_str = to_decimal_buf(num, &mut buf);

            let bytes = num_str.as_bytes();
            let len = bytes.len();

            if len & 1 == 1 {
                continue;
            }

            let mid = len / 2;
            if num_str[..mid] == num_str[mid..] {
                acc += num;
            }
        }
    }
    let elapsed = now.elapsed();
    println!("took: {:?}", elapsed);
    acc
}

fn solve_part2(ranges: Ranges) -> u64 {
    let now = Instant::now();
    let mut acc = 0;
    let mut buf = [0u8; 20]; // stack buffer, reused
    for range in ranges.0 {
        for num in range {
            let num_str = to_decimal_buf(num, &mut buf);
            let mid = num_str.len() / 2;
            for i in (1..=mid).rev() {
                let count = num_str.matches(&num_str[..i]).count();
                if count > 1 && ((i * count) == num_str.len()) {
                    acc += num;
                    break;
                }
            }
        }
    }
    let elapsed = now.elapsed();
    println!("took: {:?}", elapsed);
    acc
}

fn main() {
    let mut buf = Vec::new();
    File::open("inputs/day02.txt")
        .unwrap()
        .read_to_end(&mut buf)
        .unwrap();

    let ranges = Ranges::new(str::from_utf8(&buf).unwrap().trim());

    println!("Part 1: {}", solve_part1(ranges.clone()));
    println!("Part 2: {}", solve_part2(ranges));
}

fn to_decimal_buf(mut n: u64, out: &mut [u8; 20]) -> &str {
    // Write digits from the end backwards
    let mut i = 20;
    loop {
        i -= 1;
        out[i] = b'0' + (n % 10) as u8;
        n /= 10;
        if n == 0 {
            break;
        }
    }
    // Convert to &str
    core::str::from_utf8(&out[i..]).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::{Ranges, solve_part1, solve_part2};

    #[test]
    fn test_pt1() {
        let value = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

        let ranges = Ranges::new(value);

        let result = solve_part1(ranges);
        assert_eq!(result, 1227775554);
    }

    #[test]
    fn test_pt2() {
        let value = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

        let ranges = Ranges::new(value);

        let result = solve_part2(ranges);
        assert_eq!(result, 4174379265);
    }
}
