use std::{fs::File, io::Read, time::Instant};

use advent_of_code::utils::{BitSet, LineIterator};

// generic to make it work with the test :/
fn solve_part1_2<const WIDTH: usize, const BITFIELD_CNT: usize>(buf: &[u8]) -> u32 {
    let now = Instant::now();
    let mut lines = LineIterator::new(buf);
    let mut total = 0;

    let start_idx = lines
        .next()
        .unwrap()
        .iter()
        .position(|&c| c == b'S')
        .unwrap();
    let mut state = BitSet::<WIDTH, BITFIELD_CNT>::default();

    // add the first beam
    state.set(start_idx);

    for line in lines.skip(1).step_by(2) {
        let mut split = BitSet::default();
        for (idx, &c) in line.iter().enumerate() {
            if c == b'^' {
                // add the split
                split.set(idx);
            }
        }

        // AND find the collisions
        // (state = when there's a beam and split = when there's a splitter)
        let hits = state & split;
        total += hits.count();

        // basically combine beams that didn't hit a splitter, the left of a collision and the right of the collision
        state = (state & !split) | hits.shl1() | hits.shr1();
    }

    let elapsed = now.elapsed();
    println!("took: {:?}", elapsed);
    total
}

fn solve_part1(buf: &[u8]) -> u16 {
    let now = Instant::now();
    let mut lines = LineIterator::new(buf);
    let mut total = 0;
    let start_pos = lines
        .next()
        .unwrap()
        .iter()
        .position(|&c| c == b'S')
        .unwrap();

    let mut cur = Vec::new();
    let mut next = Vec::new();

    cur.push(start_pos);

    for line in lines.skip(1).step_by(2) {
        next.clear();
        for &idx in cur.iter() {
            if idx >= line.len() {
                continue;
            }

            match line[idx] {
                b'^' => {
                    total += 1;
                    if idx > 0 {
                        push_unique(&mut next, idx - 1);
                    }
                    if idx + 1 < line.len() {
                        push_unique(&mut next, idx + 1);
                    }
                }
                _ => {
                    push_unique(&mut next, idx);
                }
            }
        }
        if next.is_empty() {
            break;
        }
        std::mem::swap(&mut cur, &mut next);
    }
    let elapsed = now.elapsed();
    println!("took: {:?}", elapsed);
    total
}

fn solve_part2<const WIDTH: usize, const BITFIELD_CNT: usize>(buf: &[u8]) -> usize {
    let now = Instant::now();
    let mut lines = LineIterator::new(buf);
    let first_line = match lines.next() {
        Some(line) => line,
        None => return 0,
    };
    let start_pos = match first_line.iter().position(|&c| c == b'S') {
        Some(start) => start,
        None => return 0,
    };

    let mut beam_positions = BitSet::<WIDTH, BITFIELD_CNT>::default();
    beam_positions.set(start_pos);

    let mut beams_through = [0usize; WIDTH];
    let mut new_beams_through = [0usize; WIDTH];
    beams_through[start_pos] = 1;

    for line in lines.skip(1).step_by(2) {
        let mut new_beam_positions: BitSet<WIDTH, BITFIELD_CNT> = BitSet::default();
        new_beams_through.fill(0);

        for idx in beam_positions.iter_ones() {
            let count = beams_through[idx];
            if let Some(&ch) = line.get(idx) {
                if ch == b'^' {
                    if idx > 0 {
                        new_beam_positions.set(idx - 1);
                        new_beams_through[idx - 1] += count;
                    }
                    if idx + 1 < WIDTH {
                        new_beam_positions.set(idx + 1);
                        new_beams_through[idx + 1] += count;
                    }
                } else {
                    // Continue beam
                    new_beam_positions.set(idx);
                    new_beams_through[idx] += count;
                }
            }
        }

        beam_positions = new_beam_positions;
        std::mem::swap(&mut beams_through, &mut new_beams_through);
    }

    let total = beams_through.iter().sum();
    let elapsed = now.elapsed();
    println!("took: {:?}", elapsed);
    total
}

fn main() {
    let mut buf = Vec::new();
    File::open("inputs/day07.txt")
        .unwrap()
        .read_to_end(&mut buf)
        .unwrap();

    println!("Part 1: {}", solve_part1(&buf));
    println!("Part 1v2: {}", solve_part1_2::<141, 3>(&buf));
    println!("Part 2: {}", solve_part2::<141, 3>(&buf));
    //println!("Part 2: {}", solve_part2(&s));
}

#[inline(always)]
fn push_unique(deq: &mut Vec<usize>, val: usize) {
    if deq.last() != Some(&val) {
        deq.push(val);
    }
}

#[cfg(test)]
mod tests {
    use crate::{solve_part1, solve_part1_2, solve_part2};

    const INPUT: &'static str = ".......S.......\n\
    ...............\n\
    .......^.......\n\
    ...............\n\
    ......^.^......\n\
    ...............\n\
    .....^.^.^.....\n\
    ...............\n\
    ....^.^...^....\n\
    ...............\n\
    ...^.^...^.^...\n\
    ...............\n\
    ..^...^.....^..\n\
    ...............\n\
    .^.^.^.^.^...^.\n\
    ...............\n\
";

    #[test]
    fn part1() {
        let res = solve_part1(INPUT.as_bytes());
        assert_eq!(res, 21);
    }

    #[test]
    fn part1_2() {
        let res = solve_part1_2::<15, 1>(INPUT.as_bytes());
        assert_eq!(res, 21);
    }

    #[test]
    fn part2() {
        let res = solve_part2::<15, 1>(INPUT.as_bytes());
        assert_eq!(res, 40);
    }
}
