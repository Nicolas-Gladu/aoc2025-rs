use std::{fs::File, io::Read, time::Instant};

const POW10: [u64; 13] = [
    1,
    10,
    100,
    1_000,
    10_000,
    100_000,
    1_000_000,
    10_000_000,
    100_000_000,
    1_000_000_000,
    10_000_000_000,
    100_000_000_000,
    1_000_000_000_000,
];

fn solve_part1(buf: &[u8]) -> u64 {
    let now = Instant::now();
    let total = get_biggest_battery(buf, 2);
    let elapsed = now.elapsed();
    println!("took: {:?}", elapsed);
    total
}

fn solve_part2(buf: &[u8]) -> u64 {
    let now = Instant::now();
    let total = get_biggest_battery(buf, 12);
    let elapsed = now.elapsed();
    println!("took: {:?}", elapsed);
    total
}

#[inline(always)]
fn get_biggest_battery(buf: &[u8], elem_num: usize) -> u64 {
    let mut total = 0;
    for line in buf.split(|&b| b == b'\n') {
        if line.is_empty() {
            continue;
        }
        let mut idx = 0;
        for batt_idx in 0..elem_num {
            let mut max = 0;
            let inv = elem_num - batt_idx - 1;
            for i in idx..line.len() - inv {
                let n = line[i];
                if n > max {
                    max = n;
                    idx = i + 1;
                }
            }
            total += POW10[inv] * (max - b'0') as u64;
        }
    }
    total
}

fn main() {
    let mut buf = Vec::new();
    File::open("inputs/day03.txt")
        .unwrap()
        .read_to_end(&mut buf)
        .unwrap();

    let s = String::from_utf8(buf).unwrap();
    let s = s.replace("\r\n", "\n");
    let buf = s.into_bytes();

    println!("Part 1: {}", solve_part1(&buf));
    println!("Part 2: {}", solve_part2(&buf));
}

#[cfg(test)]
mod tests {
    use crate::{solve_part1, solve_part2};
    const DATA: &'static str = "987654321111111\n811111111111119\n234234234234278\n818181911112111";
    #[test]
    fn test1() {
        let result = solve_part1(DATA.as_bytes());
        assert_eq!(result, 357);
    }

    #[test]
    fn test2() {
        let result = solve_part2(DATA.as_bytes());
        assert_eq!(result, 3121910778619);
    }
}
