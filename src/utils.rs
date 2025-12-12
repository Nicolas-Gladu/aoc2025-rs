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
pub fn parse_u64(bytes: &[u8]) -> u64{
    let mut n = 0u64;
    for b in bytes {
        n = n * 10 + (b - b'0') as u64;
    }
    n
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
