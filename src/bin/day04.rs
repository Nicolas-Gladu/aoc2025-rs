use advent_of_code::utils::LineIterator;
use std::fs::File;
use std::io::Read;
use std::time::Instant;

const EMPTY: u8 = 255;

const OFFSETS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

struct Grid {
    data: Vec<u8>,
    cols: usize,
    rows: usize,
}

impl Grid {
    pub fn new(input: &[u8]) -> Self {
        let mut lines_iter = LineIterator::new(input)
            .filter(|line| !line.is_empty())
            .peekable();
        let cols = lines_iter.peek().map(|line| line.len()).unwrap_or(0);
        let rows = lines_iter.clone().count();
        let mut data = Vec::with_capacity(rows * cols);
        for byte in input {
            match byte {
                b'.' => data.push(EMPTY),
                b'@' => data.push(0),
                _ => {}
            }
        }

        Self { data, cols, rows }
    }

    #[inline(always)]
    fn refresh_neighbor(&mut self) {
        for row in 0..self.rows {
            for col in 0..self.cols {
                let c = &self.data[row * self.cols + col];
                if *c == EMPTY {
                    continue;
                }

                self.process_row(row as i32, col as i32);
            }
        }
    }

    #[inline(always)]
    fn refresh_neighbor_active(&mut self, active: &[usize]) {
        for &idx in active {
            let row = (idx / self.cols) as i32;
            let col = (idx % self.cols) as i32;

            self.process_row(row, col)
        }
    }

    #[inline(always)]
    fn process_row(&mut self, row: i32, col: i32) {
        for &(offset_row, offset_col) in &OFFSETS {
            let neighbor_row = row  + offset_row;
            let neighbor_col = col  + offset_col;

            if neighbor_row >= 0
                && neighbor_row < self.rows as i32
                && neighbor_col >= 0
                && neighbor_col < self.cols as i32
            {
                let idx = neighbor_row as usize * self.cols + neighbor_col as usize;
                if self.data[idx] != EMPTY {
                    self.data[idx] += 1;
                }
            }
        }
    }
}

fn solve_part1(buf: &[u8]) -> u32 {
    let now = Instant::now();
    let mut grid = Grid::new(buf);
    grid.refresh_neighbor();
    let result = grid
        .data
        .iter()
        .filter(|&&cell| cell < 4 && cell != EMPTY)
        .count() as u32;
    let elapsed = now.elapsed();
    println!("took: {:?}", elapsed);
    result
}

fn solve_part2(buf: &[u8]) -> u32 {
    let now = Instant::now();
    let mut grid = Grid::new(buf);
    let mut active: Vec<usize> = grid
        .data
        .iter()
        .enumerate()
        .filter(|(_, c)| **c != EMPTY)
        .map(|(i, _)| i)
        .collect();
    let mut result = 0;
    loop {
        grid.refresh_neighbor_active(&active);
        let prev_len = active.len();
        active.retain(|&idx| {
            if grid.data[idx] < 4 {
                result += 1;
                grid.data[idx] = EMPTY;
                false
            } else {
                grid.data[idx] = 0;
                true
            }
        });

        if active.len() == prev_len {
            break;
        }
    }
    let elapsed = now.elapsed();
    println!("took: {:?}", elapsed);
    result
}

fn main() {
    let mut buf = Vec::new();
    File::open("inputs/day04.txt")
        .unwrap()
        .read_to_end(&mut buf)
        .unwrap();
    println!("Part 1: {}", solve_part1(&buf));
    println!("Part 1: {}", solve_part2(&buf));
}

#[cfg(test)]
mod tests {
    use crate::{solve_part1, solve_part2};

    const DATA: &str = "..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@.\n";
    #[test]
    fn test_part1() {
        let result = solve_part1(DATA.as_bytes());
        assert_eq!(result, 13);
    }

    #[test]
    fn test_part2() {
        let result = solve_part2(DATA.as_bytes());
        assert_eq!(result, 43);
    }
}
