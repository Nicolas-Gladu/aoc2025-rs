use std::{
    fmt::Display,
    ops::{BitAnd, BitOr, Not},
};

#[derive(Clone)]
pub struct LineIterator<'a> {
    buf: &'a [u8],
    pos: usize,
}

impl<'a> LineIterator<'a> {
    pub fn new(buf: &'a [u8]) -> Self {
        Self { buf, pos: 0 }
    }
}

impl<'a> Iterator for LineIterator<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.buf.len() {
            return None;
        }

        let start = self.pos;
        while self.pos < self.buf.len() {
            if self.buf[self.pos] == b'\n' {
                let mut end = self.pos;
                if end > start && self.buf[end - 1] == b'\r' {
                    end -= 1;
                }
                self.pos += 1;
                return Some(&self.buf[start..end]);
            }
            self.pos += 1;
        }
        Some(&self.buf[start..])
    }
}

#[inline(always)]
pub fn parse_u16(bytes: &[u8]) -> u16 {
    let mut n = 0u16;
    for b in bytes {
        n = n * 10 + (b - b'0') as u16;
    }
    n
}

#[inline(always)]
pub fn parse_u32(bytes: &[u8]) -> u32 {
    let mut n = 0u32;
    for b in bytes {
        n = n * 10 + (b - b'0') as u32;
    }
    n
}

#[inline(always)]
pub fn parse_u64(bytes: &[u8]) -> u64 {
    let mut n = 0u64;
    for b in bytes {
        n = n * 10 + (b - b'0') as u64;
    }
    n
}

#[derive(Clone, Copy)]
pub struct BitSet<const WIDTH: usize, const N: usize>([u64; N]);

// googled this part and converted to rust lol
impl<const WIDTH: usize, const N: usize> BitSet<WIDTH, N> {
    const LAST_IDX: usize = N - 1;
    const LAST_MASK: u64 = if WIDTH % 64 == 0 {
        u64::MAX
    } else {
        (1u64 << (WIDTH % 64)) - 1
    };

    #[inline]
    pub fn set(&mut self, idx: usize) {
        self.0[idx >> 6] |= 1 << (idx & 63)
    }

    // equivalent to shif left (left beam) accross 3 u64
    #[inline]
    pub fn shl1(self) -> Self {
        let mut new = Self::default();
        for i in 0..N {
            new.0[i] = self.0[i] << 1;
            if i > 0 {
                new.0[i] |= self.0[i - 1] >> 63;
            }
        }
        new.0[Self::LAST_IDX] &= Self::LAST_MASK;
        new
    }

    // equivalent to shif right (right beam) accross 3 u64
    #[inline]
    pub fn shr1(self) -> Self {
        let mut new = Self::default();
        for i in 0..N {
            new.0[i] = self.0[i] >> 1;
            if i < Self::LAST_IDX {
                new.0[i] |= self.0[i + 1] << 63;
            }
        }
        new.0[0] &= !1;
        new
    }

    #[inline]
    fn get(&self, idx: usize) -> bool {
        (self.0[idx >> 6] & (1 << (idx & 63))) != 0
    }

    #[inline]
    pub fn iter_ones(&self) -> impl Iterator<Item = usize> + '_ {
        (0..WIDTH).filter(move |&i| self.get(i))
    }

    #[inline]
    pub fn count(&self) -> u32 {
        self.0.iter().map(|x| x.count_ones()).sum()
    }
}

impl<const WIDTH: usize, const N: usize> Default for BitSet<WIDTH, N> {
    fn default() -> Self {
        Self([0; N])
    }
}

// AND all the bitfields in one go
impl<const WIDTH: usize, const N: usize> BitAnd for BitSet<WIDTH, N> {
    type Output = Self;

    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        let mut result = Self::default();
        for i in 0..N {
            result.0[i] = self.0[i] & rhs.0[i];
        }
        result
    }
}

// OR all the bitfields in one go
impl<const WIDTH: usize, const N: usize> BitOr for BitSet<WIDTH, N> {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        let mut result = Self::default();
        for i in 0..N {
            result.0[i] = self.0[i] | rhs.0[i];
        }
        result
    }
}

// NOT all the bitfield
impl<const WIDTH: usize, const N: usize> Not for BitSet<WIDTH, N> {
    type Output = Self;

    #[inline]
    fn not(self) -> Self::Output {
        let mut result = Self::default();
        for i in 0..N {
            result.0[i] = !self.0[i];
        }
        result.0[Self::LAST_IDX] &= Self::LAST_MASK;
        result
    }
}

impl<const WIDTH: usize, const N: usize> Display for BitSet<WIDTH, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..N {
            write!(f, "{:b}", self.0[i])?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::LineIterator;

    const RESULT: [&'static [u8]; 4] = [
        b"987654321111111",
        b"811111111111119",
        b"234234234234278",
        b"818181911112111",
    ];
    #[test]
    fn test_unix() {
        const DATA: &'static str =
            "987654321111111\n811111111111119\n234234234234278\n818181911112111";
        let iter = LineIterator::new(DATA.as_bytes());
        assert_eq!(&RESULT, iter.collect::<Vec<_>>().as_slice());
    }

    #[test]
    fn test_windows() {
        const DATA: &'static str =
            "987654321111111\r\n811111111111119\r\n234234234234278\r\n818181911112111";
        let iter = LineIterator::new(DATA.as_bytes());
        assert_eq!(&RESULT, iter.collect::<Vec<_>>().as_slice());
    }
}
